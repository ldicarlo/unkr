use crate::brute_force_state;
use crate::brute_force_state::apply_decrypt;
use crate::cache;
use crate::console::PrintableMessage;
use crate::console::ThreadStatusPayload;
use crate::models;
use crate::models::BruteForceCryptor;
use crate::models::BruteForceState;
use crate::models::DoneLine;
use crate::models::PermuteBruteForceState;
use crate::models::VigenereBruteForceState;
use std::collections::BTreeMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front

pub fn start(
    thread_count: usize,
    combinations: Vec<Vec<BruteForceCryptor>>,
    clues: Vec<String>,
    strings: Vec<String>,
    cache_args: models::CacheArgs,
    candidates_sender: Sender<(Vec<String>, Vec<String>, String)>,
    console_sender: Sender<PrintableMessage>,
) {
    let thread_work =
        start_thread_work(combinations, clues.clone(), strings.clone()).expect("Nothing to do.");
    let am_tw = Arc::new(Mutex::new(thread_work));
    let (thread_status_sender, thread_status_receiver) = channel();
    for i in 0..thread_count {
        let local_tw = am_tw.clone();
        let local_sender = thread_status_sender.clone();
        let local_cache_args = cache_args.clone();
        let local_clues = clues.clone();
        let local_strings = strings.clone();
        let local_candidates_sender = candidates_sender.clone();
        let local_console_sender = console_sender.clone();
        thread::spawn(move || {
            run_thread_work(
                local_sender,
                i,
                local_tw,
                local_cache_args,
                local_clues,
                local_strings,
                local_candidates_sender,
                local_console_sender,
            )
        });
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
    println!("was {:?} for {:?}", vec, done_line);
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
        None
    }
}

fn get_cryptor_from_state(brute_force_state: &BruteForceState) -> BruteForceCryptor {
    match brute_force_state {
        BruteForceState::Vigenere(VigenereBruteForceState {
            brute_force_args,
            args: _,
        }) => BruteForceCryptor::Vigenere(*brute_force_args),
        BruteForceState::Cut(_) => BruteForceCryptor::Cut,
        BruteForceState::Caesar(_) => BruteForceCryptor::Caesar,
        BruteForceState::Transpose(_) => BruteForceCryptor::Transpose,
        BruteForceState::AtBash => BruteForceCryptor::AtBash,
        BruteForceState::Reverse => BruteForceCryptor::Reverse,
        BruteForceState::Swap(_) => BruteForceCryptor::Swap,
        BruteForceState::Join => BruteForceCryptor::Join,
        BruteForceState::Permute(PermuteBruteForceState {
            brute_force_args,
            args: _,
        }) => BruteForceCryptor::Permute(brute_force_args.clone()),
        BruteForceState::Enigma(_) => BruteForceCryptor::Enigma,
    }
}

fn run_thread_work(
    sender: Sender<()>,
    thread_number: usize,
    tw: Arc<Mutex<ThreadWork>>,
    cache_args: models::CacheArgs,
    clues: Vec<String>,
    strings: Vec<String>,
    candidates_sender: Sender<(Vec<String>, Vec<String>, String)>,
    console_sender: Sender<PrintableMessage>,
) {
    let mut step = 0;
    loop {
        if let Some(new_tw) = lock_and_increase(tw.clone()) {
            step = step + 1;
            console_sender
                .send(PrintableMessage::ThreadStatus(ThreadStatusPayload {
                    thread_number,
                    step,
                    total: 100,
                    current_combination: new_tw.current_head.clone(),
                }))
                .unwrap();
            let first = apply_decrypt(new_tw.current_head.clone(), strings.clone());
            if first.len() > 0 {
                let acc =
                    brute_force_state::get_name(&get_cryptor_from_state(&new_tw.current_head));
                candidates_sender
                    .send((first.clone(), clues.clone(), acc.clone()))
                    .unwrap();
                brute_force_state::loop_decrypt(
                    Some(acc),
                    new_tw.current_tail.clone(),
                    first,
                    clues.clone(),
                    candidates_sender.clone(),
                );
            }
            thread_combination_over(new_tw.current_combination, tw.clone(), cache_args.clone());
        } else {
            break;
        }
    }
    sender.send(()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::BruteForceVigenereArgs;
    use crate::vigenere;
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
                current_tail: vec![BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                    alphabet_depth: 1,
                    key_depth: 2
                }),],
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
