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
pub fn internal_brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors_filtered: Vec<BruteForceCryptor>,
    threads: u8,
    done_cache: BTreeSet<DoneLine>,
    cache_args: CacheArgs,
    combinations: HashSet<Vec<u8>>,
) -> BTreeSet<String> {
    let mut internal_combinations = combinations.clone();

    while internal_combinations.len() > 0 {
        while let Some(next) = next() {}
    }

    BTreeSet::new()
}

fn next() -> Option<CryptorTypeWithArgs> {
    None
}

// Thread 1 (Send thread status -> thread_system | Receive work to do)
// Thread 2 (Send thread status -> thread_system | Receive work to do)
// thread_system sends work to do
// push_done

pub fn brute_force_combination(
    head: BruteForceCryptor,
    tail: Vec<BruteForceCryptor>,
    thread_assignments: Vec<std::sync::mpsc::Sender<String>>,
    thread_status: Vec<std::sync::mpsc::Receiver<bool>>,
) {
    while let Some(next) = next() {}
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWork {
    pub head: Option<ThreadWorkHead>,
    pub remaining_combinations: Vec<Vec<BruteForceCryptor>>,
    pub working_combinations: BTreeMap<PartialCombination, Vec<()>>,
    pub clues: Vec<String>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum BruteForceState {
    Vigenere(VigenereBruteForceState),
    Cut(NumberArgs),
    Caesar(NumberArgs),
    Transpose(NumberArgs),
    AtBash,
    Reverse,
    Swap(SwapArgs),
    Join,
    Colors(StringArgs),
    IndexCrypt(StringArgs),
    Permute(PermuteBruteForceState),
    Enigma(EnigmaArgs),
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct VigenereBruteForceState {
    pub brute_force_args: BruteForceVigenereArgs,
    pub args: VigenereArgs,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct PermuteBruteForceState {
    pub brute_force_args: BruteForcePermuteArgs,
    pub args: PermuteArgs,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWorkHead {
    pub last_head: BruteForceState,
    pub last_tail: Vec<BruteForceCryptor>,
    pub partial_combination: PartialCombination,
}

pub fn start(
    thread_count: usize,
    tw: Arc<Mutex<ThreadWork>>,
    //     done_cache: BTreeSet<DoneLine>,
    // cache_args: CacheArgs,
) {
    // let state = Arc::new(Mutex::new(ThreadWork {
    //     head: None, //?
    //     remaining_combinations: 1,
    //     working_combinations: HashMap::new(),
    // }));

    for i in 0..thread_count {
        //let (done_line_sender, done_line_receiver) = channel();
        let local_tw = tw.clone();
        thread::spawn(move || thread_work(local_tw.clone()));
        //thread::spawn(move || thread_combination_over(local_tw.clone(), done_line_sender));
    }
}

// fn loop_decrypt(
//   acc: Option<String>,
//   mut to_use: Vec<u8>,
//   strs: Vec<String>,
//   clues: Vec<String>,
//   decryptors_filtered: Vec<BruteForceCryptor>,
//   cache_args: models::CacheArgs,
//   candidates_sender: std::sync::mpsc::Sender<(Vec<String>, Vec<String>, String)>,
// )

fn thread_combination_over(partial_done_line: DoneLine, tw: Arc<Mutex<ThreadWork>>) {
    // tw.working_combination.done_line.pop()
    // if head != done_line && tw.working_combination.done_line empty
    // push_done
    // remove tw.working_combination.done_line
}

fn thread_work(tw: Arc<Mutex<ThreadWork>>) {
    loop {
        let current = if let Ok(thread_work) = tw.lock() {
            if let Some(current_head) = thread_work.head.clone() {
                //   push to working_combinations
                let partial_combination = current_head.partial_combination;
                let mut vec = thread_work
                    .working_combinations
                    .get(&partial_combination)
                    .map(|x| x.clone())
                    .unwrap_or(vec![]);
                vec.push(());
                thread_work
                    .working_combinations
                    .insert(partial_combination, vec);
            } else {
            }
            None
        } else {
            None
        };
        // loop_decrypt(None);
        println!("Stuff done");
        if let Some(work_to_do) = current {
            //  thread_combination_over(partial_done_line, tw);
        } else {
            break;
        }
    }
}

fn make_next(state: BruteForceState) -> Option<BruteForceState> {
    match state {
        BruteForceState::Vigenere(_) => todo!(),
        BruteForceState::Cut(_) => todo!(),
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
