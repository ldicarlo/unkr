use super::combinator;
use crate::atbash;
use crate::brute_force;
use crate::brute_force_state;
use crate::cache;
use crate::caesar;
use crate::candidates;
use crate::console;
use crate::console::ThreadStatusPayload;
use crate::cryptors;
use crate::cut;
use crate::dispatcher;
use crate::enigma;
use crate::enigma::EnigmaArgs;
use crate::join;
use crate::models;
use crate::models::BruteForceCryptor;
use crate::models::BruteForcePermuteArgs;
use crate::models::BruteForceState;
use crate::models::BruteForceVigenereArgs;
use crate::models::CacheArgs;
use crate::models::Cryptor;
use crate::models::NumberArgs;
use crate::models::PermuteArgs;
use crate::models::StringArgs;
use crate::models::SwapArgs;
use crate::models::VigenereArgs;
use crate::models::VigenereBruteForceState;
use crate::models::{CryptorTypeWithArgs, DoneLine};
use crate::parser;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::transpose;
use crate::vigenere;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front
// Thread 1 (Send thread status -> thread_system | Receive work to do)
// Thread 2 (Send thread status -> thread_system | Receive work to do)
// thread_system sends work to do
// push_done

pub fn start(
    thread_count: usize,
    combinations: Vec<Vec<BruteForceCryptor>>,
    clues: Vec<String>,
    strings: Vec<String>,
    cache_args: models::CacheArgs,
) {
    let thread_work = start_thread_work(combinations, clues, strings).expect("Nothing to do.");
    println!("{:?}", thread_work);
    let am_tw = Arc::new(Mutex::new(thread_work));
    let (thread_status_sender, thread_status_receiver) = channel();
    for i in 0..thread_count {
        let local_tw = am_tw.clone();
        let local_sender = thread_status_sender.clone();
        let local_cache_args = cache_args.clone();
        thread::spawn(move || run_thread_work(local_sender, i, local_tw, local_cache_args));
    }

    for _ in 0..thread_count {
        thread_status_receiver.recv().unwrap();
    }
}

fn thread_combination_over(
    done_line: DoneLine,
    tw: Arc<Mutex<ThreadWork>>,
    cache_args: models::CacheArgs,
) {
    let mut thread_work = tw.lock().unwrap();
    let mut vec = thread_work
        .working_combinations
        .get(&done_line)
        .unwrap()
        .clone();
    vec.pop();

    if vec.len() <= 0 && done_line != thread_work.current_combination {
        cache::push_done(done_line.clone(), cache_args);
        thread_work.working_combinations.remove(&done_line);
    } else {
        thread_work.working_combinations.insert(done_line, vec);
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWork {
    pub current_head: BruteForceState,
    pub current_tail: Vec<BruteForceCryptor>,
    pub current_combination: DoneLine,
    pub remaining_combinations: Vec<Vec<BruteForceCryptor>>,
    pub working_combinations: BTreeMap<DoneLine, Vec<()>>,
    pub clues: Vec<String>,
    pub strings: Vec<String>,
}

fn start_thread_work(
    combinations: Vec<Vec<BruteForceCryptor>>,
    clues: Vec<String>,
    strings: Vec<String>,
) -> Option<ThreadWork> {
    let mut remaining_combinations = combinations.clone();
    remaining_combinations.pop().and_then(|x| {
        let mut popable = x.clone();
        popable.pop().map(|bfc| {
            let current_head = brute_force_state::start_state(bfc);
            ThreadWork {
                current_head,
                current_tail: popable,
                current_combination: cache::to_done(x),
                remaining_combinations,
                working_combinations: BTreeMap::new(),
                clues,
                strings,
            }
        })
    })
}

fn increase_thread_work(
    ThreadWork {
        current_head,
        current_tail,
        current_combination,
        remaining_combinations,
        working_combinations,
        clues,
        strings,
    }: ThreadWork,
) -> Option<ThreadWork> {
    brute_force_state::increase_state(current_head)
        .map(|new_head| ThreadWork {
            current_head: new_head,
            current_tail: current_tail.clone(),
            current_combination: current_combination.clone(),
            remaining_combinations: remaining_combinations.clone(),
            working_combinations: working_combinations.clone(),
            clues: clues.clone(),
            strings: strings.clone(),
        })
        .or({
            let mut mut_remaining_combinations = remaining_combinations.clone();
            let maybe_new_current_combination = mut_remaining_combinations.pop();
            maybe_new_current_combination.map(|new_current_combination| {
                let mut mut_new_current_combination = new_current_combination.clone();
                let new_head = mut_new_current_combination.pop().unwrap();
                ThreadWork {
                    current_head: brute_force_state::start_state(new_head),
                    current_tail,
                    current_combination: cache::to_done(new_current_combination),
                    remaining_combinations: mut_remaining_combinations,
                    working_combinations,
                    clues,
                    strings,
                }
            })
        })
}

fn add_working_combination(
    ThreadWork {
        current_head,
        current_tail,
        current_combination,
        remaining_combinations,
        working_combinations,
        clues,
        strings,
    }: ThreadWork,
) -> ThreadWork {
    let mut vec = working_combinations
        .get(&current_combination)
        .map(|x| x.clone())
        .unwrap_or(vec![]);
    vec.push(());
    let mut new_working_combinations = working_combinations.clone();
    new_working_combinations.insert(current_combination.clone(), vec);
    ThreadWork {
        current_head,
        current_tail,
        current_combination,
        remaining_combinations,
        working_combinations: new_working_combinations,
        clues,
        strings,
    }
}

fn lock_and_increase(tw: Arc<Mutex<ThreadWork>>) -> Option<ThreadWork> {
    let mut thread_work = tw.lock().unwrap();
    if let Some(next_thread_work) = increase_thread_work(thread_work.clone()) {
        *thread_work = add_working_combination(next_thread_work);
        Some(thread_work.clone())
    } else {
        println!("got none in {} with {:?}", 0, thread_work.clone());
        None
    }
}

fn run_thread_work(
    sender: Sender<()>,
    thread_number: usize,
    tw: Arc<Mutex<ThreadWork>>,
    cache_args: models::CacheArgs,
) {
    println!("Spawned Thread {}", thread_number);
    loop {
        if let Some(new_tw) = lock_and_increase(tw.clone()) {
            thread_combination_over(new_tw.current_combination, tw.clone(), cache_args.clone());

            // decrypt

            println!("Stuff done {}", thread_number);
        } else {
            break;
        }
    }
    println!("Finished Thread {}", thread_number);
    sender.send(()).unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn increase_thread_works() {
        assert_eq!(
            Some(ThreadWork {
                clues: vec![String::from("hello")],
                current_combination: DoneLine {
                    args: Some(String::from("Vigenere:1:1")),
                    combinations: String::from("Vigenere Join")
                },
                current_head: BruteForceState::Join,
                current_tail: vec![],
                remaining_combinations: vec![],
                working_combinations: BTreeMap::new(),
                strings: vec![String::from("ENCRYPTED")]
            }),
            super::increase_thread_work(ThreadWork {
                current_combination: DoneLine {
                    args: None,
                    combinations: String::from("")
                },
                current_head: BruteForceState::Join,
                current_tail: vec![],
                clues: vec![String::from("hello")],
                remaining_combinations: vec![vec![
                    BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                        alphabet_depth: 1,
                        key_depth: 1
                    }),
                    BruteForceCryptor::Join
                ]],
                working_combinations: super::BTreeMap::new(),
                strings: vec![String::from("ENCRYPTED")]
            })
        );
    }

    #[test]
    fn start_thread_works() {
        assert_eq!(
            Some(ThreadWork {
                clues: vec![String::from("hello")],
                current_combination: DoneLine {
                    args: Some(String::from("Vigenere:1:2")),
                    combinations: String::from("Join Vigenere")
                },
                current_head: BruteForceState::Vigenere(VigenereBruteForceState {
                    args: vigenere::init(),
                    brute_force_args: BruteForceVigenereArgs {
                        alphabet_depth: 1,
                        key_depth: 2
                    }
                }),
                current_tail: vec![BruteForceCryptor::Join],
                remaining_combinations: vec![],
                working_combinations: vec![].into_iter().collect(),
                strings: vec![String::from("ENCRYPTED")]
            }),
            super::start_thread_work(
                vec![vec![
                    BruteForceCryptor::Join,
                    BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                        alphabet_depth: 1,
                        key_depth: 2
                    }),
                ]],
                vec![String::from("hello")],
                vec![String::from("ENCRYPTED")],
            )
        );
    }

    #[test]
    fn add_working_combination() {
        assert_eq!(
            ThreadWork {
                clues: vec![String::from("hello")],
                current_combination: DoneLine {
                    args: Some(String::from("Vigenere:1:2")),
                    combinations: String::from("Join Vigenere")
                },
                current_head: BruteForceState::Vigenere(VigenereBruteForceState {
                    args: vigenere::init(),
                    brute_force_args: BruteForceVigenereArgs {
                        alphabet_depth: 1,
                        key_depth: 2
                    }
                }),
                current_tail: vec![BruteForceCryptor::Join],
                remaining_combinations: vec![],
                working_combinations: vec![(
                    DoneLine {
                        args: Some(String::from("Vigenere:1:2")),
                        combinations: String::from("Join Vigenere")
                    },
                    vec![(), ()]
                )]
                .into_iter()
                .collect(),
                strings: vec![String::from("ENCRYPTED")]
            },
            super::add_working_combination(super::add_working_combination(ThreadWork {
                clues: vec![String::from("hello")],
                current_combination: DoneLine {
                    args: Some(String::from("Vigenere:1:2")),
                    combinations: String::from("Join Vigenere")
                },
                current_head: BruteForceState::Vigenere(VigenereBruteForceState {
                    args: vigenere::init(),
                    brute_force_args: BruteForceVigenereArgs {
                        alphabet_depth: 1,
                        key_depth: 2
                    }
                }),
                current_tail: vec![BruteForceCryptor::Join],
                remaining_combinations: vec![],
                working_combinations: vec![].into_iter().collect(),
                strings: vec![String::from("ENCRYPTED")]
            }))
        )
    }
}
