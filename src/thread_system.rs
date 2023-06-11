use clap::Args;

use super::combinator;
use crate::atbash;
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
use crate::models::{CryptorTypeWithArgs, DoneLine, PartialCombination};
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWork {
    pub head: Option<ThreadWorkHead>,
    pub remaining_combinations: Vec<Vec<BruteForceCryptor>>,
    pub working_combinations: BTreeMap<PartialCombination, Vec<()>>,
    pub clues: Vec<String>,
    pub strings: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWorkHead {
    pub last_head: BruteForceState,
    pub last_tail: Vec<BruteForceCryptor>,
    pub partial_combination: PartialCombination,
}

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

fn increase_thread_work(thread_work: ThreadWork) -> Option<ThreadWork> {
    thread_work
        .clone()
        .head
        .clone()
        .and_then(|head| {
            match head.clone().last_head {
                BruteForceState::Vigenere(state) => vigenere::next(state.clone()).map(|args| {
                    {
                        models::BruteForceState::Vigenere(models::VigenereBruteForceState {
                            args,
                            brute_force_args: state.brute_force_args,
                        })
                    }
                }),
                BruteForceState::Cut(args) => todo!(),
                BruteForceState::Caesar(_) => todo!(),
                BruteForceState::Transpose(_) => todo!(),
                BruteForceState::AtBash => None,
                BruteForceState::Reverse => None,
                BruteForceState::Swap(_) => todo!(),
                BruteForceState::Join => None,
                BruteForceState::Permute(_) => todo!(),
                BruteForceState::Enigma(_) => None,
            }
            .map(|x| (head, x, thread_work.remaining_combinations.clone()))
        })
        .map(|(head, state, remainings)| ThreadWorkHead {
            last_head: state,
            last_tail: head.last_tail,
            partial_combination: head.partial_combination,
        })
        .or({
            let mut remaining_combinations = thread_work.clone().remaining_combinations.clone();
            let maybe_combination: Option<Vec<BruteForceCryptor>> = remaining_combinations.pop();
            maybe_combination.map(|combination| ThreadWork {
                head: None,
                remaining_combinations,
                clues: thread_work.clues,
                working_combinations: thread_work.working_combinations,
                strings: thread_work.strings,
            })
        })
        .map()
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
                head: None,
                remaining_combinations: vec![],
                working_combinations: vec![(
                    PartialCombination {
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
                clues: vec![String::from("hello")],
                head: None,
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
