use super::combinator;
use crate::atbash;
use crate::cache;
use crate::caesar;
use crate::candidates;
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
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::vec;

pub fn brute_force_unique_combination(
    str: String,
    clues: Vec<String>,
    decryptors: Vec<String>,
    threads: u8,
    cache_name: String,
) {
    let decr: Vec<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();
    let cache_args = cache::prepare_cache_args(cache_name.clone(), str.clone(), clues.clone());
    eprintln!("{:?}", decr);

    let combination: Vec<u8> = decr
        .clone()
        .iter()
        .enumerate()
        .map(|(i, _)| i as u8)
        .rev()
        .collect();

    loop_decrypt(
        Arc::new(Mutex::new(BTreeSet::new())),
        None,
        combination.clone(),
        vec![str],
        clues,
        decr.clone(),
        cache_args.clone(),
    );
    cache::push_done(cache::to_done(decr, combination), cache_args.clone());
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
    eprintln!("{:?}", decr);
    let result = brute_force_strings(str, clues, steps, decr, threads, done_cache, cache_args);
    eprintln!("Result: {:?}", result);
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
        let done_line = cache::to_done(decryptors.clone(), vec.clone());
        if skip_combination(vec.clone(), decryptors.clone()) {
            eprintln!(
                "SKIPPED combination: {:?}",
                parse_combination(vec.clone(), decryptors.clone())
            );
        } else if cache::already_done(done_cache.clone(), done_line.clone()) {
            eprintln!("CACHE present: {:?}", done_line.clone())
        } else {
            filtered_combinations.push(vec);
        }
    }

    let threads_work = dispatcher::dispatch(threads as usize, filtered_combinations.len());
    eprintln!("TOTAL combinations {}", filtered_combinations.clone().len(),);
    eprintln!("THREADS dispatching {:?}", threads_work.clone(),);
    eprintln!("DECRYPTORS {}", decryptors.clone().len(),);

    let mut vec_combinations = filtered_combinations
        .clone()
        .into_iter()
        .collect::<Vec<Vec<u8>>>();

    vec_combinations.shuffle(&mut rand::thread_rng());

    let (sender, receiver) = channel();
    for (i, t) in threads_work.into_iter().enumerate() {
        let local_combinations = vec_combinations.split_off(vec_combinations.len() - t);

        let local_sender = sender.clone();
        let local_results_accumulator = results_accumulator.clone();
        let local_str = str.clone();
        let local_clues = clues.clone();
        let local_decryptors = decryptors.clone();
        let local_cache_args = cache_args.clone();
        eprintln!(
            "THREAD {}\tstart {} combinations",
            i,
            local_combinations.len()
        );
        thread::spawn(move || {
            local_sender
                .send(threaded_function(
                    i,
                    local_combinations,
                    local_results_accumulator,
                    local_str,
                    local_clues,
                    local_decryptors,
                    local_cache_args,
                ))
                .unwrap();
        });
    }
    for t in 0..threads {
        eprintln!("THREAD {}\tfinished({})", t, receiver.recv().unwrap());
    }

    let result: BTreeSet<String> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn threaded_function(
    thread_number: usize,
    combinations: Vec<Vec<u8>>,
    results_accumulator: Arc<Mutex<BTreeSet<String>>>,
    str: String,
    clues: Vec<String>,
    decryptors_filtered: Vec<BruteForceCryptor>,
    cache_args: models::CacheArgs,
) -> bool {
    for (i, vec) in combinations.iter().enumerate() {
        let done_line = cache::to_done(decryptors_filtered.clone(), vec.clone());
        eprintln!(
            "THREAD {}\tcombination: {} {:?}",
            thread_number,
            i,
            done_line.clone()
        );

        loop_decrypt(
            results_accumulator.clone(),
            None,
            vec.clone(),
            vec![str.clone()],
            clues.clone(),
            decryptors_filtered.clone(),
            cache_args.clone(),
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
    res_acc: Arc<Mutex<BTreeSet<String>>>,
    acc: Option<String>,
    mut to_use: Vec<u8>,
    strs: Vec<String>,
    clues: Vec<String>,
    decryptors_filtered: Vec<BruteForceCryptor>,
    cache_args: models::CacheArgs,
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
                    res_acc.clone(),
                    acc,
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                    cache_args.clone(),
                );

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    cache_args.clone(),
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
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                        cache_args.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
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
                    res_acc.clone(),
                    acc,
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                    cache_args.clone(),
                );

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    cache_args.clone(),
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
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                        cache_args.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                    );
                }
            }
            BruteForceCryptor::Vigenere(brute_force_vigenere_args) => {
                let mut current_args = vigenere::init();
                let cryptor_name = String::from("Vigenere");
                while let Some(next) =
                    vigenere::next(current_args.clone(), brute_force_vigenere_args)
                {
                    let new_str = vigenere::decrypt(strs.clone(), next.clone());
                    if strs == new_str {
                        continue;
                    }
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}:{}", next.key, next.alphabet),
                        new_str.clone(),
                        cache_args.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
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
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                        cache_args.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
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
                    res_acc.clone(),
                    acc.clone(),
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                    cache_args.clone(),
                );

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    cache_args.clone(),
                );
            }
            BruteForceCryptor::Permute(args) => {
                let mut current_permutations = permute::init();
                while let Some(next) = permute::next(current_permutations.clone(), args.clone()) {
                    eprintln!("{:?}", next);
                    let new_str = permute::decrypt(strs.clone(), next.clone());

                    if strs == new_str {
                        continue;
                    }

                    let cryptor_name = String::from("Permute");
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_permutations),
                        new_str.clone(),
                        cache_args.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
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
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_order),
                        new_str.clone(),
                        cache_args.clone(),
                    );
                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                    );
                    current_order = next;
                }
            }
            BruteForceCryptor::IndexCrypt => todo!(),
            BruteForceCryptor::Enigma => {
                let mut current_args = enigma::init();
                while let Some(next) = enigma::next(current_args.clone()) {
                    let new_str = enigma::decrypt(strs.clone(), next.clone());
                    let cryptor_name = String::from("Enigma");
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_args),
                        new_str.clone(),
                        cache_args.clone(),
                    );
                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        cache_args.clone(),
                    );
                    current_args = next;
                }
            }
        }
    }
}

fn process_new_str(
    res_acc: Arc<Mutex<BTreeSet<String>>>,
    acc: Option<String>,
    clues: Vec<String>,
    // mut cache: BTreeSet<(Vec<String>, Vec<u8>, u64)>,
    cryptor_str: String,
    new_str: Vec<String>,
    cache_args: models::CacheArgs,
) -> Option<String> {
    let current_acc = acc
        .clone()
        .map(|existing| existing + " " + &cryptor_str.clone())
        .unwrap_or(cryptor_str.clone());
    let candidates =
        candidates::find_and_print_candidates(new_str.clone(), clues.clone(), current_acc.clone());

    if candidates.len() > 0 {
        let local_arc = res_acc.clone();
        local_arc.lock().unwrap().insert(current_acc.clone());
        cache::push_hit(
            cache_args,
            models::HitLine {
                args: current_acc.clone(),
                result: new_str.clone().join(""),
            },
        )
    }
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
                BruteForceCryptor::IndexCrypt => false,
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
