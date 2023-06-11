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
}

fn thread_combination_over(partial_done_line: DoneLine, tw: Arc<Mutex<ThreadWork>>) {
    // tw.working_combination.done_line.pop()
    // if head != done_line && tw.working_combination.done_line empty
    // push_done
    // remove tw.working_combination.done_line
}

fn increase_thread_work(thread_work: ThreadWork) -> Option<ThreadWork> {
    if let Some(head) = thread_work.head.clone() {
        match head.last_head {};
        None
    } else {
        let mut remaining_combinations = thread_work.remaining_combinations.clone();
        let maybe_combination: Option<Vec<BruteForceCryptor>> = remaining_combinations.pop();
        maybe_combination.map(|combination| ThreadWork {
            head: None,
            remaining_combinations,
            clues: thread_work.clues,
            working_combinations: thread_work.working_combinations,
            strings: thread_work.strings,
        })
    }
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

fn make_next(state: BruteForceState) -> Option<BruteForceState> {
    match state {
        BruteForceState::Vigenere(state) => vigenere::next(state).map(|args| {
            models::BruteForceState::Vigenere(models::VigenereBruteForceState {
                args,
                brute_force_args: state.brute_force_args,
            })
        }),
        BruteForceState::Cut(args) => todo!(),
        BruteForceState::Caesar(_) => todo!(),
        BruteForceState::Transpose(_) => todo!(),
        BruteForceState::AtBash => todo!(),
        BruteForceState::Reverse => todo!(),
        BruteForceState::Swap(_) => todo!(),
        BruteForceState::Join => todo!(),
        BruteForceState::Colors(_) => todo!(),
        BruteForceState::IndexCrypt(_) => todo!(),
        BruteForceState::Permute(_) => todo!(),
        BruteForceState::Enigma(_) => todo!(),
    }
}
