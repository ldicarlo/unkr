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
use std::collections::BTreeSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
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
    let total = combinations.len();
    let thread_work =
        start_thread_work(combinations, clues.clone(), strings.clone()).expect("Nothing to do.");
    let am_tw = Arc::new(Mutex::new(ThreadsStatuses {
        workload: BTreeMap::new(),
    }));
    let (thread_status_sender, thread_status_receiver) = channel();
    let (thread_combination_status_sender, thread_combination_status_receiver) = channel();
    let done_cache = cache::get_done_cache(cache_args.clone());
    for i in 0..thread_count {
        let local_tw = thread_work.clone();
        let local_sender = thread_status_sender.clone();
        let local_clues = clues.clone();
        let local_strings = strings.clone();
        let local_candidates_sender = candidates_sender.clone();
        let local_console_sender = console_sender.clone();
        let local_done_cache = done_cache.clone();
        let local_combination_status_sender = thread_combination_status_sender.clone();
        thread::spawn(move || {
            run_thread_work(
                local_sender,
                i,
                thread_count,
                local_tw,
                local_clues,
                local_strings,
                local_candidates_sender,
                local_console_sender,
                total,
                local_done_cache,
                local_combination_status_sender,
            )
        });
    }

    thread::spawn(move || {
        thread_combination_status_function(thread_combination_status_receiver, am_tw, cache_args);
    });

    for _ in 0..thread_count {
        thread_status_receiver.recv().unwrap();
    }
}

fn thread_combination_status_function(
    r: Receiver<ThreadStatus>,
    tw: Arc<Mutex<ThreadsStatuses>>,
    cache_args: models::CacheArgs,
) {
    r.iter().for_each(|status| {
        thread_combination_status(status, tw.clone(), cache_args.clone());
    });
}

enum ThreadStatus {
    Doing(usize, DoneLine),
    Done(usize, DoneLine),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum WorkStatus {
    Doing,
    Done,
}

fn apply_state(
    thread_status: ThreadStatus,
    state: ThreadsStatuses,
) -> (ThreadsStatuses, Option<DoneLine>) {
    let mut mutable_state = state.clone();
    match thread_status {
        ThreadStatus::Doing(thread_number, done_line) => {
            let current = state
                .workload
                .get(&thread_number)
                .map(|(_, b)| (Some(done_line.clone()), b.clone()))
                .unwrap_or((Some(done_line), vec![]));
            mutable_state.workload.insert(thread_number, current);
            (mutable_state, None)
        }
        ThreadStatus::Done(thread_number, done_line) => {
            let current = state
                .workload
                .get(&thread_number)
                .map(|(_, b)| {
                    let mut mutable_vec = b.clone();
                    mutable_vec.push(done_line.clone());

                    (None, mutable_vec)
                })
                .unwrap_or((None, vec![done_line.clone()]));
            mutable_state.workload.insert(thread_number, current);

            let done = if mutable_state
                .workload
                .iter()
                .all(|(_, (_, v))| v.contains(&done_line))
            {
                Some(done_line.clone())
            } else {
                None
            };

            (mutable_state, done)
        }
    }
}

fn thread_combination_status(
    thread_status: ThreadStatus,
    tw: Arc<Mutex<ThreadsStatuses>>,
    cache_args: models::CacheArgs,
) {
    let mut state = tw.lock().unwrap();

    let (result, done_to_push) = apply_state(thread_status, state.clone());

    if let Some(done) = done_to_push {
        cache::push_done(done, cache_args);
    }

    *state = result;
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWork {
    pub current_head: BruteForceState,
    pub current_tail: Vec<BruteForceCryptor>,
    pub current_combination: DoneLine,
    pub remaining_combinations: Vec<Vec<BruteForceCryptor>>,
    pub clues: Vec<String>,
    pub strings: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadsStatuses {
    pub workload: BTreeMap<usize, (Option<DoneLine>, Vec<DoneLine>)>,
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
        clues,
        strings,
    }: ThreadWork,
) -> Option<ThreadWork> {
    brute_force_state::increase_state(current_head, strings.clone())
        .map(|new_head| ThreadWork {
            current_head: new_head,
            current_tail: current_tail.clone(),
            current_combination: current_combination.clone(),
            remaining_combinations: remaining_combinations.clone(),
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
                    clues,
                    strings,
                }
            })
        })
}

// fn add_working_combination(
//     ThreadsStatuses {
//         current_combination,
//         working_combinations,
//     }: ThreadsStatuses,
// ) -> ThreadsStatuses {
//     let (done, mut vec) = working_combinations
//         .get(&current_combination)
//         .map(|x| x.clone())
//         .unwrap_or((WorkStatus::Doing, vec![]));
//     vec.push(());
//     let mut new_working_combinations = working_combinations.clone();
//     new_working_combinations.insert(current_combination.clone(), (done, vec));
//     ThreadsStatuses {
//         current_combination,
//         working_combinations: new_working_combinations,
//     }
// }

// fn done_combination(
//     ThreadsStatuses {
//         current_combination,
//         working_combinations,
//     }: ThreadsStatuses,
// ) -> ThreadsStatuses {
//     let (_, vec) = working_combinations
//         .get(&current_combination)
//         .map(|x| x.clone())
//         .unwrap_or((WorkStatus::Doing, vec![]));
//     let mut new_working_combinations = working_combinations.clone();
//     new_working_combinations.insert(current_combination.clone(), (WorkStatus::Done, vec));
//     ThreadsStatuses {
//         current_combination,
//         working_combinations: new_working_combinations,
//     }
// }

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
    thread_count: usize,
    mut tw: ThreadWork,
    clues: Vec<String>,
    strings: Vec<String>,
    candidates_sender: Sender<(Vec<String>, Vec<String>, String)>,
    console_sender: Sender<PrintableMessage>,
    total: usize,
    done_cache: BTreeSet<models::DoneLine>,
    combination_status_sender: Sender<ThreadStatus>,
) {
    let mut step = 0;

    loop {
        if let Some(new_tw) = increase_thread_work(tw.clone()) {
            tw = new_tw.clone();
            step = step + 1;

            if step % thread_count != thread_number {
                continue;
            }
            if cache::already_done(done_cache.clone(), new_tw.current_combination.clone()) {
                continue;
            }

            console_sender
                .send(PrintableMessage::ThreadStatus(ThreadStatusPayload {
                    thread_number,
                    step,
                    total,
                    current_combination: new_tw.current_head.clone(),
                }))
                .unwrap();

            if new_tw.current_combination != tw.clone().current_combination {
                combination_status_sender
                    .send(ThreadStatus::Done(
                        thread_number,
                        tw.current_combination.clone(),
                    ))
                    .unwrap();
                combination_status_sender
                    .send(ThreadStatus::Doing(
                        thread_number,
                        new_tw.current_combination.clone(),
                    ))
                    .unwrap();
            }

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
        } else {
            combination_status_sender
                .send(ThreadStatus::Done(
                    thread_number,
                    tw.current_combination.clone(),
                ))
                .unwrap();
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
                current_tail: vec![BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                    alphabet_depth: 1,
                    key_depth: 2
                }),],
                remaining_combinations: vec![],
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

    // #[test]
    // fn add_working_combination() {
    //     assert_eq!(
    //         ThreadsStatuses {
    //             current_combination: DoneLine {
    //                 args: Some(String::from("Vigenere:1:2")),
    //                 combinations: String::from("Join Vigenere")
    //             },

    //             working_combinations: vec![(
    //                 DoneLine {
    //                     args: Some(String::from("Vigenere:1:2")),
    //                     combinations: String::from("Join Vigenere")
    //                 },
    //                 (WorkStatus::Doing, vec![(), ()])
    //             )]
    //             .into_iter()
    //             .collect(),
    //         },
    //         super::add_working_combination(super::add_working_combination(ThreadsStatuses {
    //             current_combination: DoneLine {
    //                 args: Some(String::from("Vigenere:1:2")),
    //                 combinations: String::from("Join Vigenere")
    //             },
    //             working_combinations: vec![].into_iter().collect(),
    //         }))
    //     )
    // }

    #[test]
    fn apply_state_works() {
        assert_eq!(
            (
                ThreadsStatuses {
                    workload: vec![(
                        1,
                        (
                            Some(DoneLine {
                                args: None,
                                combinations: String::from("Join")
                            }),
                            vec![]
                        ),
                    )]
                    .into_iter()
                    .collect()
                },
                None
            ),
            apply_state(
                ThreadStatus::Doing(
                    1,
                    DoneLine {
                        args: None,
                        combinations: String::from("Join")
                    },
                ),
                ThreadsStatuses {
                    workload: BTreeMap::new()
                }
            )
        );
    }

    #[test]
    fn apply_state_works_2() {
        assert_eq!(
            (
                ThreadsStatuses {
                    workload: vec![(
                        1,
                        (
                            None,
                            vec![DoneLine {
                                args: None,
                                combinations: String::from("Join")
                            }]
                        ),
                    )]
                    .into_iter()
                    .collect()
                },
                Some(DoneLine {
                    args: None,
                    combinations: String::from("Join")
                })
            ),
            apply_state(
                ThreadStatus::Done(
                    1,
                    DoneLine {
                        args: None,
                        combinations: String::from("Join")
                    },
                ),
                ThreadsStatuses {
                    workload: BTreeMap::new()
                }
            )
        );
    }
}
