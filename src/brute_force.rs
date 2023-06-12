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
use crate::join;
use crate::models;
use crate::models::BruteForceCryptor;
use crate::models::CacheArgs;
use crate::parser;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::transpose;
use crate::vigenere;
use rand::prelude::SliceRandom;
use std::collections::BTreeSet;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::vec;

pub fn brute_force_unique_combination(
    str: String,
    clues: Vec<String>,
    decryptors: Vec<String>,
    _threads: u8,
    cache_name: String,
) {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decr: Vec<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();
    let cache_args = cache::prepare_cache_args(cache_name.clone(), str.clone(), clues.clone());
    // eprintln!("{:?}", decr);

    let combination: Vec<u8> = decr
        .clone()
        .iter()
        .enumerate()
        .map(|(i, _)| i as u8)
        .rev()
        .collect();
    let (candidates_sender, candidates_receiver) = channel();
    let (console_sender, console_receiver) = channel();
    let local_cache_args = cache_args.clone();
    thread::spawn(move || {
        candidates::candidate_receiver(
            candidates_receiver,
            local_cache_args,
            results_accumulator.clone(),
            console_sender,
        )
    });

    loop_decrypt(
        None,
        combination.clone(),
        vec![str],
        clues,
        decr.clone(),
        cache_args.clone(),
        candidates_sender.clone(),
    );
    cache::push_done(
        cache::to_done_from_combination(decr, combination),
        cache_args.clone(),
    );
}

pub fn brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors: Vec<String>,
    threads: u8,
    cache_name: String,
) {
    let decr: Vec<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();
    let cache_args = cache::prepare_cache_args(cache_name.clone(), str.clone(), clues.clone());
    let done_cache = cache::get_done_cache(cache_args.clone());
    // eprintln!("{:?}", decr);
    let result = brute_force_strings(str, clues, steps, decr, threads, done_cache, cache_args);
    //eprintln!("Result: {:?}", result);
}

pub fn internal_brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors_filtered: Vec<BruteForceCryptor>,
    threads: u8,
    done_cache: BTreeSet<models::DoneLine>,
    cache_args: CacheArgs,
) -> BTreeSet<String> {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decryptors = if decryptors_filtered.len() == 0 {
        cryptors::get_decryptors()
    } else {
        decryptors_filtered
    };
    let combinations = combinator::combine_elements(decryptors.len().try_into().unwrap(), steps);

    // discard combinations
    let mut filtered_combinations = Vec::new();
    for vec in combinations.into_iter() {
        let done_line = cache::to_done_from_combination(decryptors.clone(), vec.clone());
        if skip_combination(vec.clone(), decryptors.clone()) {
            // eprintln!(
            //     "SKIPPED combination: {:?}",
            //     parse_combination(vec.clone(), decryptors.clone())
            // );
        } else if cache::already_done(done_cache.clone(), done_line.clone()) {
            // eprintln!("CACHE present: {:?}", done_line.clone())
        } else {
            filtered_combinations.push(vec);
        }
    }

    let threads_work = dispatcher::dispatch(threads as usize, filtered_combinations.len());
    //  eprintln!("TOTAL combinations {}", filtered_combinations.clone().len(),);
    //  eprintln!("THREADS dispatching {:?}", threads_work.clone(),);
    //  eprintln!("DECRYPTORS {}", decryptors.clone().len(),);

    let mut vec_combinations = filtered_combinations
        .clone()
        .into_iter()
        .collect::<Vec<Vec<u8>>>();

    vec_combinations.shuffle(&mut rand::thread_rng());

    let (sender, receiver) = channel();
    let (candidates_sender, candidates_receiver) = channel();
    let (console_sender, console_receiver) = channel();
    let local_cache_args = cache_args.clone();
    let local_results_accumulator = results_accumulator.clone();
    let local_console_sender = console_sender.clone();
    thread::spawn(move || {
        candidates::candidate_receiver(
            candidates_receiver,
            local_cache_args,
            local_results_accumulator.clone(),
            local_console_sender.clone(),
        )
    });
    thread::spawn(move || console::thread_consume_messages(console_receiver, threads as usize));

    for (i, t) in threads_work.into_iter().enumerate() {
        let local_combinations = vec_combinations.split_off(vec_combinations.len() - t);

        let local_sender = sender.clone();
        let local_str = str.clone();
        let local_clues = clues.clone();
        let local_decryptors = decryptors.clone();
        let local_cache_args = cache_args.clone();
        let local_candidate_sender = candidates_sender.clone();
        let local_console_sender = console_sender.clone();

        // eprintln!(
        //     "THREAD {}\tstart {} combinations",
        //     i,
        //     local_combinations.len()
        // );
        thread::spawn(move || {
            local_sender
                .send(threaded_function(
                    i,
                    local_combinations,
                    local_str,
                    local_clues,
                    local_decryptors,
                    local_cache_args,
                    local_candidate_sender,
                    local_console_sender,
                ))
                .unwrap();
        });
    }
    for _ in 0..threads {
        //eprintln!("THREAD {}\tfinished({})", t, receiver.recv().unwrap());
        receiver.recv().unwrap();
    }

    let result: BTreeSet<String> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn threaded_function(
    thread_number: usize,
    combinations: Vec<Vec<u8>>,
    str: String,
    clues: Vec<String>,
    decryptors_filtered: Vec<BruteForceCryptor>,
    cache_args: models::CacheArgs,
    candidates_sender: Sender<(Vec<String>, Vec<String>, String)>,
    console_sender: Sender<console::PrintableMessage>,
) -> bool {
    let total = combinations.len();
    for (i, vec) in combinations.iter().enumerate() {
        let done_line = cache::to_done_from_combination(decryptors_filtered.clone(), vec.clone());
        console_sender
            .send(console::PrintableMessage::ThreadStatus(
                ThreadStatusPayload {
                    thread_number,
                    step: i,
                    total,
                    current_combination: done_line.combinations.clone(),
                },
            ))
            .unwrap();
        // eprintln!(
        //     "THREAD {}\tcombination: {} {:?}",
        //     thread_number,
        //     i,
        //     done_line.clone()
        // );

        loop_decrypt(
            None,
            vec.clone(),
            vec![str.clone()],
            clues.clone(),
            decryptors_filtered.clone(),
            cache_args.clone(),
            candidates_sender.clone(),
        );

        cache::push_done(done_line.clone(), cache_args.clone());
    }
    true
}

fn brute_force_strings(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors_filtered: Vec<BruteForceCryptor>,
    threads: u8,
    done_cache: BTreeSet<models::DoneLine>,
    cache_args: CacheArgs,
) -> BTreeSet<String> {
    internal_brute_force_decrypt(
        str,
        clues,
        steps,
        decryptors_filtered.clone(),
        threads,
        done_cache,
        cache_args,
    )
}

fn loop_decrypt(
    acc: Option<String>,
    mut to_use: Vec<u8>,
    strs: Vec<String>,
    clues: Vec<String>,
    decryptors_filtered: Vec<BruteForceCryptor>,
    cache_args: models::CacheArgs,
    candidates_sender: std::sync::mpsc::Sender<(Vec<String>, Vec<String>, String)>,
) {
    if let Some(current) = to_use.pop() {
        let cryptor = decryptors_filtered
            .clone()
            .into_iter()
            .nth(current.into())
            .unwrap();

        match cryptor {
            BruteForceCryptor::AtBash => {
                let new_str: Vec<String> = atbash::decrypt(strs.clone());
                if strs == new_str {
                    return;
                }
                let cryptor_name = String::from("AtBash");
                let current_acc = process_new_str(
                    acc,
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                    candidates_sender.clone(),
                );

                loop_decrypt(
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    cache_args.clone(),
                    candidates_sender.clone(),
                );
            }
            BruteForceCryptor::Caesar => {
                for s in 0..26 {
                    let new_str = caesar::decrypt(strs.clone(), models::NumberArgs { number: s });
                    let cryptor_name = String::from("Caesar");

                    if strs == new_str {
                        continue;
                    }

                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );

                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                }
            }
            BruteForceCryptor::Reverse => {
                let new_str = reverse::decrypt(strs.clone());
                if strs == new_str {
                    return;
                }
                let cryptor_name = String::from("Reverse");
                let current_acc = process_new_str(
                    acc,
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                    candidates_sender.clone(),
                );

                loop_decrypt(
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    cache_args.clone(),
                    candidates_sender.clone(),
                );
            }
            BruteForceCryptor::Transpose => {
                for s in 1..strs.first().map(|s| s.len()).unwrap_or(0) {
                    let new_str =
                        transpose::decrypt(strs.clone(), models::NumberArgs { number: s });
                    if strs == new_str {
                        continue;
                    }
                    let cryptor_name = String::from("Transpose");

                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );

                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                }
            }
            BruteForceCryptor::Vigenere(brute_force_args) => {
                let mut current_args = vigenere::init();
                let cryptor_name = String::from("Vigenere");
                while let Some(next) = vigenere::next(models::VigenereBruteForceState {
                    args: current_args.clone(),
                    brute_force_args,
                }) {
                    let new_str = vigenere::decrypt(strs.clone(), next.clone());
                    if strs == new_str {
                        continue;
                    }
                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}:{}", next.key, next.alphabet),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );

                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                    current_args = next;
                }
            }
            BruteForceCryptor::Cut => {
                for s in 0..strs.first().map(|s| s.len()).unwrap_or(0) {
                    let new_str = cut::encrypt(strs.clone(), models::NumberArgs { number: s });
                    if strs == new_str {
                        continue;
                    }
                    let cryptor_name = String::from("Cut");
                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );

                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                }
            }
            BruteForceCryptor::Join => {
                if strs.len() == 1 {
                    return;
                }

                let new_str = join::decrypt(strs.clone());
                let cryptor_name = String::from("Join");
                let current_acc = process_new_str(
                    acc.clone(),
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                    candidates_sender.clone(),
                );

                loop_decrypt(
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    cache_args.clone(),
                    candidates_sender.clone(),
                );
            }
            BruteForceCryptor::Permute(args) => {
                let mut current_permutations = permute::init();
                while let Some(next) = permute::next(models::PermuteBruteForceState {
                    args: current_permutations.clone(),
                    brute_force_args: args.clone(),
                }) {
                    //  eprintln!("{:?}", next);
                    let new_str = permute::decrypt(strs.clone(), next.clone());

                    if strs == new_str {
                        continue;
                    }

                    let cryptor_name = String::from("Permute");
                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_permutations),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );

                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                    current_permutations = next;
                }
            }
            BruteForceCryptor::Swap => {
                let mut current_order = swap::init();
                while let Some(next) = swap::next(current_order.clone(), strs.len()) {
                    let new_str = swap::decrypt(strs.clone(), next.clone());
                    let cryptor_name = String::from("Swap");
                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_order),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );
                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                    current_order = next;
                }
            }
            //      BruteForceCryptor::IndexCrypt => todo!(),
            BruteForceCryptor::Enigma => {
                let mut current_args = enigma::init();
                while let Some(next) = enigma::next(current_args.clone()) {
                    let new_str = enigma::decrypt(strs.clone(), next.clone());
                    let cryptor_name = String::from("Enigma");
                    let current_acc = process_new_str(
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_args),
                        new_str.clone(),
                        candidates_sender.clone(),
                    );
                    loop_decrypt(
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                        candidates_sender.clone(),
                    );
                    current_args = next;
                }
            }
        }
    }
}

fn process_new_str(
    acc: Option<String>,
    clues: Vec<String>,
    cryptor_str: String,
    new_str: Vec<String>,

    candidates_sender: std::sync::mpsc::Sender<(Vec<String>, Vec<String>, String)>,
) -> Option<String> {
    let current_acc = acc
        .clone()
        .map(|existing| existing + " " + &cryptor_str.clone())
        .unwrap_or(cryptor_str.clone());
    candidates_sender
        .send((new_str.clone(), clues.clone(), current_acc.clone()))
        .unwrap();

    Some(current_acc)
}

fn skip_combination(combination: Vec<u8>, cryptors: Vec<BruteForceCryptor>) -> bool {
    let cryp_combination = parse_combination(combination, cryptors);
    let not_first = vec![Some(&BruteForceCryptor::Join)];
    if not_first.contains(&cryp_combination.first()) {
        return true;
    }

    let not_last = vec![
        Some(&BruteForceCryptor::Join),
        Some(&BruteForceCryptor::Cut),
    ];
    if not_last.contains(&cryp_combination.last()) {
        return true;
    }
    let mut last = None;

    for next in cryp_combination.into_iter() {
        if last
            .clone()
            .map(|prev: BruteForceCryptor| match next.clone() {
                BruteForceCryptor::Vigenere(_) => false,
                BruteForceCryptor::Cut => false,
                BruteForceCryptor::Caesar => false, // maybe yes
                BruteForceCryptor::Transpose => false,
                BruteForceCryptor::AtBash => atbash::skip_if_previous_in().contains(&prev),
                BruteForceCryptor::Reverse => reverse::skip_if_previous_in().contains(&prev),
                BruteForceCryptor::Swap => swap::skip_if_previous_in().contains(&prev),
                BruteForceCryptor::Join => join::skip_if_previous_in().contains(&prev),
                //   BruteForceCryptor::IndexCrypt => false,
                BruteForceCryptor::Permute(args) => {
                    permute::skip_if_previous_in(args).contains(&prev)
                }
                BruteForceCryptor::Enigma => enigma::skip_if_previous_in().contains(&prev),
            })
            .unwrap_or(false)
        {
            return true;
        }
        last = Some(next);
    }

    false
}

fn parse_combination(
    combination: Vec<u8>,
    cryptors: Vec<BruteForceCryptor>,
) -> Vec<BruteForceCryptor> {
    combination
        .into_iter()
        .map(|c| (cryptors.clone().into_iter().nth(c.into()).unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::cache::{prepare_cache_args, tests::test_cache_name};

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            BTreeSet::new(),
            internal_brute_force_decrypt(
                "str".to_string(),
                vec![
                    "CLOCK".to_string(),
                    "BERLIN".to_string(),
                    "NORTH".to_string(),
                    "EAST".to_string(),
                ],
                2,
                vec![BruteForceCryptor::Caesar, BruteForceCryptor::AtBash],
                1,
                BTreeSet::new(),
                prepare_cache_args(
                    test_cache_name(),
                    "str".to_string(),
                    vec![
                        "CLOCK".to_string(),
                        "BERLIN".to_string(),
                        "NORTH".to_string(),
                        "EAST".to_string(),
                    ],
                )
            )
        );
    }

    #[test]
    fn brute_force_1() {
        assert_eq!(
            // encrypt --string BERLIN -- caesar:10 atbash caesar:5
            brute_force_strings(
                "TQDJMH".to_string(),
                vec![
                    "CLOCK".to_string(),
                    "BERLIN".to_string(),
                    "NORTH".to_string(),
                    "EAST".to_string(),
                ],
                2,
                vec![BruteForceCryptor::Caesar, BruteForceCryptor::AtBash],
                1,
                BTreeSet::new(),
                prepare_cache_args(
                    test_cache_name(),
                    "TQDJMH".to_string(),
                    vec![
                        "CLOCK".to_string(),
                        "BERLIN".to_string(),
                        "NORTH".to_string(),
                        "EAST".to_string(),
                    ],
                )
            ),
            vec![
                "Caesar:5 AtBash".to_string(),
                "AtBash Caesar:21".to_string(),
            ]
            .into_iter()
            .collect::<BTreeSet<String>>(),
        );
    }

    #[test]
    fn skip_combination_works() {
        assert_eq!(
            skip_combination(
                vec![1, 0, 0],
                vec![
                    models::BruteForceCryptor::Permute(models::BruteForcePermuteArgs {
                        max_permutations: 4
                    },),
                    models::BruteForceCryptor::AtBash
                ]
            ),
            true
        )
    }
}
