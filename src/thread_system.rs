use clap::Args;

use super::combinator;
use crate::atbash;
use crate::brute_force;
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

pub fn start(thread_count: usize, tw: Arc<Mutex<ThreadWork>>) {
    for i in 0..thread_count {
        let local_tw = tw.clone();
        thread::spawn(move || thread_work(local_tw.clone()));
    }

    for i in 0..thread_count {
        // receive end signal
    }
}

fn thread_combination_over(partial_done_line: DoneLine, tw: Arc<Mutex<ThreadWork>>) {
    // tw.working_combination.done_line.pop()
    // if head != done_line && tw.working_combination.done_line empty
    // push_done
    // remove tw.working_combination.done_line
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

fn start_state(brute_force_cryptor: BruteForceCryptor) -> BruteForceState {
    match brute_force_cryptor {
        BruteForceCryptor::Vigenere(brute_force_args) => {
            BruteForceState::Vigenere(VigenereBruteForceState {
                brute_force_args,
                args: vigenere::init(),
            })
        }
        BruteForceCryptor::Cut => todo!(),
        BruteForceCryptor::Caesar => BruteForceState::Caesar(caesar::init()),
        BruteForceCryptor::Transpose => todo!(),
        BruteForceCryptor::AtBash => todo!(),
        BruteForceCryptor::Reverse => todo!(),
        BruteForceCryptor::Swap => todo!(),
        BruteForceCryptor::Join => BruteForceState::Join,
        //   BruteForceCryptor::IndexCrypt => todo!(),
        BruteForceCryptor::Permute(_) => todo!(),
        BruteForceCryptor::Enigma => todo!(),
    }
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
            let current_head = start_state(bfc);

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

fn increase_thread_work(thread_work: ThreadWork) -> Option<ThreadWork> {
    // thread_work
    //     .clone()
    //     .and_then(|head| {
    //         match head.clone().last_head {
    //             BruteForceState::Vigenere(state) => vigenere::next(state.clone()).map(|args| {
    //                 {
    //                     models::BruteForceState::Vigenere(models::VigenereBruteForceState {
    //                         args,
    //                         brute_force_args: state.brute_force_args,
    //                     })
    //                 }
    //             }),
    //             BruteForceState::Cut(args) => todo!(),
    //             BruteForceState::Caesar(_) => todo!(),
    //             BruteForceState::Transpose(_) => todo!(),
    //             BruteForceState::AtBash => None,
    //             BruteForceState::Reverse => None,
    //             BruteForceState::Swap(_) => todo!(),
    //             BruteForceState::Join => None,
    //             BruteForceState::Permute(_) => todo!(),
    //             BruteForceState::Enigma(_) => None,
    //         }
    //         .map(|x| (head, x, thread_work.remaining_combinations.clone()))
    //     })
    //     .map(|(head, state, remainings)| (state, head.last_tail, head.partial_combination))
    //     .or({
    //         let mut remaining_combinations = thread_work.clone().remaining_combinations.clone();
    //         let maybe_combination: Option<Vec<BruteForceCryptor>> = remaining_combinations.pop();
    //         maybe_combination.map(|combination| ())
    //     })
    //     .map(|(a, b, c)| ThreadWork {
    //         clues: thread_work.clues,
    //         head: Some(twh),
    //         remaining_combinations: thread_work.remaining_combinations,
    //         strings: thread_work.strings,
    //         working_combinations: thread_work.working_combinations,
    //     })
    None
}

fn thread_work(tw: Arc<Mutex<ThreadWork>>) {
    // loop {
    //     if let Ok(thread_work) = tw.lock() {
    //         if let Some(next_thread_work) = increase_thread_work(thread_work) {
    //             //   push to working_combinations
    //             let partial_combination = vec![];
    //             let mut vec = thread_work
    //                 .working_combinations
    //                 .get(&partial_combination)
    //                 .map(|x| x.clone())
    //                 .unwrap_or(vec![]);
    //             vec.push(());
    //             thread_work
    //                 .working_combinations
    //                 .insert(partial_combination, vec);
    //         } else {
    //         }
    //         None
    //     } else {
    //         None
    //     };
    //     // loop_decrypt(None);
    //     println!("Stuff done");
    //     if let Some(work_to_do) = current {
    //         //  thread_combination_over(partial_done_line, tw);
    //     } else {
    //         break;
    //     }
    // }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Some(ThreadWork {
                clues: vec![String::from("hello")],
                current_combination: DoneLine {
                    args: None,
                    combinations: String::from("")
                },
                current_head: BruteForceState::Join,
                current_tail: vec![],
                remaining_combinations: vec![],
                working_combinations: vec![(
                    DoneLine {
                        combinations: String::from(""),
                        args: None
                    },
                    vec![()]
                )]
                .into_iter()
                .collect(),
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
}
