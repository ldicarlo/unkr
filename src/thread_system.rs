use crate::brute_force_state;
use crate::brute_force_state::apply_decrypt;
use crate::cache;
use crate::candidates;
use crate::console;
use crate::console::PrintableMessage;
use crate::console::ThreadStatusPayload;
use crate::mapper;
use crate::models;
use crate::models::BruteForceCryptor;
use crate::models::BruteForceState;
use crate::models::Cryptor;
use crate::models::DoneLine;
use crate::models::PartialLine;
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front

pub fn start(
    str: String,
    thread_count: usize,
    combinations: Vec<VecDeque<BruteForceCryptor>>,
    clues: Vec<String>,
    pretty: bool,
    cache_name: String,
) {
    let strings = vec![str.clone()];
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));

    let cache_args = cache::prepare_cache_args(cache_name.clone(), str.clone(), clues.clone());
    let (candidates_sender, candidates_receiver) = unbounded();
    let (console_sender, console_receiver) = unbounded();
    let local_cache_args = cache_args.clone();
    thread::spawn(move || console::thread_consume_messages(console_receiver, thread_count));

    let local_console_sender = console_sender.clone();
    let local_results_accumulator = results_accumulator.clone();
    thread::spawn(move || {
        candidates::candidate_receiver(
            candidates_receiver,
            local_cache_args,
            local_results_accumulator.clone(),
            local_console_sender,
            pretty,
        )
    });
    let thread_work =
        start_thread_work(combinations, clues.clone(), strings.clone()).expect("Nothing to do.");
    let am_tw = Arc::new(Mutex::new(ThreadsStatuses {
        workload: BTreeMap::new(),
    }));
    let (thread_status_sender, thread_status_receiver) = unbounded();
    let (thread_combination_status_sender, thread_combination_status_receiver) = unbounded();

    let done_cache = cache::get_done_cache(cache_args.clone());
    let partial_cache = cache::get_partial_cache(cache_args.clone());
    for i in 0..thread_count {
        thread_combination_status_sender
            .send(ThreadStatus::Doing(
                i,
                thread_work.clone().current_combination,
            ))
            .unwrap();
        let local_tw = thread_work.clone();
        let local_sender = thread_status_sender.clone();
        let local_clues = clues.clone();
        let local_strings = strings.clone();
        let local_candidates_sender = candidates_sender.clone();
        let local_console_sender = console_sender.clone();
        let local_done_cache = done_cache.clone();
        let local_combination_status_sender = thread_combination_status_sender.clone();
        let local_partial_cache = partial_cache.clone();
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
                local_done_cache,
                local_partial_cache,
                local_combination_status_sender,
            )
        });
    }
    drop(thread_combination_status_sender);

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
    while let Ok(status) = r.recv() {
        thread_combination_status(status, tw.clone(), cache_args.clone());
    }
}

enum ThreadStatus {
    Doing(usize, DoneLine),
    Done(usize, DoneLine),
    DonePartial(PartialLine),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum WorkStatus {
    Doing,
    Done,
}

fn apply_state(
    thread_status: ThreadStatus,
    state: ThreadsStatuses,
) -> (ThreadsStatuses, Option<DoneLine>, Option<PartialLine>) {
    let mut mutable_state = state.clone();
    match thread_status {
        ThreadStatus::Doing(thread_number, done_line) => {
            let current = state
                .workload
                .get(&thread_number)
                .map(|(_, b)| (Some(done_line.clone()), b.clone()))
                .unwrap_or((Some(done_line), vec![]));
            mutable_state.workload.insert(thread_number, current);
            (mutable_state, None, None)
        }
        ThreadStatus::Done(thread_number, done_line) => {
            let current = state
                .workload
                .get(&thread_number)
                .map(|(_, b)| {
                    let mut mutable_vec: Vec<DoneLine> = b.clone();
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

            (mutable_state, done, None)
        }
        ThreadStatus::DonePartial(partial_line) => (state, None, Some(partial_line)),
    }
}

fn thread_combination_status(
    thread_status: ThreadStatus,
    tw: Arc<Mutex<ThreadsStatuses>>,
    cache_args: models::CacheArgs,
) {
    let mut state = tw.lock().unwrap();

    let (result, done_to_push, partial_to_push) = apply_state(thread_status, state.clone());

    if let Some(done) = done_to_push {
        cache::push_done(done, cache_args.clone());
    }
    if let Some(partial) = partial_to_push {
        cache::push_partial(partial, cache_args.clone());
    }

    *state = result;
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadWork {
    pub current_head: BruteForceState,
    pub current_tail: VecDeque<BruteForceCryptor>,
    pub current_combination: DoneLine,
    pub remaining_combinations: Vec<VecDeque<BruteForceCryptor>>,
    pub clues: Vec<String>,
    pub strings: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct ThreadsStatuses {
    pub workload: BTreeMap<usize, (Option<DoneLine>, Vec<DoneLine>)>,
}

fn start_thread_work(
    combinations: Vec<VecDeque<BruteForceCryptor>>,
    clues: Vec<String>,
    strings: Vec<String>,
) -> Option<ThreadWork> {
    let mut remaining_combinations = combinations.clone();
    remaining_combinations.pop().and_then(|x| {
        let mut popable = x.clone();
        popable.pop_front().map(|bfc| {
            let current_head = brute_force_state::start_state(bfc);
            ThreadWork {
                current_head,
                current_tail: popable,
                current_combination: mapper::to_done(x),
                remaining_combinations,
                clues,
                strings,
            }
        })
    })
}

fn cached_increase_thread_work(
    tw: ThreadWork,
    done_cache: BTreeSet<models::DoneLine>,
) -> Option<ThreadWork> {
    increase_thread_work(tw).and_then(|opt| {
        if cache::already_done(done_cache.clone(), opt.clone().current_combination) {
            println!("{:?}", opt.clone().current_combination);
            increase_combination(opt.remaining_combinations, opt.clues, opt.strings)
                .and_then(|n| cached_increase_thread_work(n, done_cache.clone()))
        } else {
            Some(opt)
        }
    })
}

fn increase_combination(
    remaining_combinations: Vec<VecDeque<BruteForceCryptor>>,
    clues: Vec<String>,
    strings: Vec<String>,
) -> Option<ThreadWork> {
    let mut mut_remaining_combinations = remaining_combinations.clone();
    let maybe_new_current_combination = mut_remaining_combinations.pop();
    maybe_new_current_combination.map(|new_current_combination| {
        let mut mut_new_current_combination = new_current_combination.clone();
        let new_head = mut_new_current_combination.pop_front().unwrap();
        ThreadWork {
            current_head: brute_force_state::start_state(new_head),
            current_tail: mut_new_current_combination,
            current_combination: mapper::to_done(new_current_combination),
            remaining_combinations: mut_remaining_combinations,
            clues,
            strings,
        }
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
    brute_force_state::increase_state(current_head.clone(), strings.clone())
        .map(|new_head| ThreadWork {
            current_head: new_head,
            current_tail: current_tail.clone(),
            current_combination: current_combination.clone(),
            remaining_combinations: remaining_combinations.clone(),
            clues: clues.clone(),
            strings: strings.clone(),
        })
        .or(increase_combination(remaining_combinations, clues, strings))
}

fn run_thread_work(
    sender: Sender<()>,
    thread_number: usize,
    thread_count: usize,
    mut tw: ThreadWork,
    clues: Vec<String>,
    strings: Vec<String>,
    candidates_sender: Sender<(Vec<String>, Vec<String>, Vec<Cryptor>)>,
    console_sender: Sender<PrintableMessage>,
    done_cache: BTreeSet<models::DoneLine>,
    partial_cache: BTreeSet<models::PartialLine>,
    combination_status_sender: Sender<ThreadStatus>,
) {
    let mut step = 0;

    loop {
        if let Some(new_tw) = cached_increase_thread_work(tw.clone(), done_cache.clone()) {
            tw = new_tw.clone();
            step = step + 1;

            if step % thread_count != thread_number {
                continue;
            }
            let partial_line = mapper::to_partial(
                brute_force_state::get_cryptor(&new_tw.current_head, vec![]),
                new_tw.current_tail.clone(),
            );
            if cache::partial_done(partial_cache.clone(), partial_line.clone()) {
                continue;
            }
            console_sender
                .send(PrintableMessage::ThreadStatus(ThreadStatusPayload {
                    thread_number,
                    step,
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

            let first = apply_decrypt(new_tw.current_head.clone(), strings.clone(), vec![]);
            if first.len() > 0 {
                let acc = vec![brute_force_state::get_cryptor(&new_tw.current_head, vec![])];
                candidates_sender
                    .send((first.clone(), clues.clone(), acc.clone()))
                    .unwrap();
                brute_force_state::loop_decrypt(
                    acc,
                    new_tw.current_tail.clone(),
                    first,
                    clues.clone(),
                    candidates_sender.clone(),
                );
            }

            combination_status_sender
                .send(ThreadStatus::DonePartial(partial_line.clone()))
                .unwrap();
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
    use pretty_assertions::assert_eq;
    #[test]
    fn increase_thread_works() {
        assert_eq!(
            Some(ThreadWork {
                clues: vec![String::from("hello")],
                current_combination: DoneLine {
                    args: Some(String::from("Vigenere:1:1")),
                    combinations: String::from("Join Vigenere")
                },
                current_head: BruteForceState::Join,
                current_tail: vec![BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                    alphabet_depth: 1,
                    key_depth: 1
                }),]
                .into(),
                remaining_combinations: vec![],
                strings: vec![String::from("ENCRYPTED")]
            }),
            super::increase_thread_work(ThreadWork {
                current_combination: DoneLine {
                    args: None,
                    combinations: String::from("")
                },
                current_head: BruteForceState::Join,
                current_tail: vec![].into(),
                clues: vec![String::from("hello")],
                remaining_combinations: vec![vec![
                    BruteForceCryptor::Join,
                    BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                        alphabet_depth: 1,
                        key_depth: 1
                    }),
                ]
                .into()]
                .into(),
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
                current_head: BruteForceState::Join,
                current_tail: vec![BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                    alphabet_depth: 1,
                    key_depth: 2
                })]
                .into(),
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
                ]
                .into()],
                vec![String::from("hello")],
                vec![String::from("ENCRYPTED")],
            )
        );
    }

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
                None,
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
                }),
                None
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
