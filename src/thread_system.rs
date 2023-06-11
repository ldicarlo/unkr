use crate::models::{
    BruteForceCryptor, CacheArgs, CryptorTypeWithArgs, DoneLine, PartialCombination,
};
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
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct ThreadWork {
    pub head: Option<ThreadWorkHead>,
    pub remaining_combinations: u8,
    pub working_combinations: BTreeMap<PartialCombination, Vec<()>>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct ThreadWorkHead {
    pub last_head: u8,
    pub last_tail: u8,
    pub partial_combination: PartialCombination,
}

pub fn start(thread_count: usize, tw: Arc<Mutex<ThreadWork>>) {
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
        //   loop_encrypt
        if let Some(work_to_do) = current {
            //  thread_combination_over(partial_done_line, tw);
        } else {
            break;
        }
    }
}
