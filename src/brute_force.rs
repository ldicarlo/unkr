use super::combinator;
use crate::atbash;
use crate::cache;
use crate::candidates;
use crate::console;
use crate::enigma;
use crate::join;
use crate::models;
use crate::models::BruteForceCryptor;
use crate::parser;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::thread_system;
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
    threads_count: u8,
    cache_name: String,
) {
    // todo join that in next function
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decr: Vec<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();
    let cache_args = cache::prepare_cache_args(cache_name.clone(), str.clone(), clues.clone());
    let (candidates_sender, candidates_receiver) = channel();
    let (console_sender, console_receiver) = channel();
    let local_cache_args = cache_args.clone();
    thread::spawn(move || {
        console::thread_consume_messages(console_receiver, threads_count as usize)
    });

    let local_console_sender = console_sender.clone();
    let local_results_accumulator = results_accumulator.clone();
    thread::spawn(move || {
        candidates::candidate_receiver(
            candidates_receiver,
            local_cache_args,
            local_results_accumulator.clone(),
            local_console_sender,
        )
    });
    thread_system::start(
        threads_count as usize,
        vec![decr],
        clues,
        vec![str],
        cache_args,
        candidates_sender,
        console_sender,
    );

    eprintln!("Result: {:?}", results_accumulator.lock().unwrap());
}

pub fn brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors: Vec<String>,
    threads_count: u8,
    cache_name: String,
) {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decr: Vec<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();
    let cache_args = cache::prepare_cache_args(cache_name.clone(), str.clone(), clues.clone());
    let (candidates_sender, candidates_receiver) = channel();
    let (console_sender, console_receiver) = channel();
    let local_cache_args = cache_args.clone();
    thread::spawn(move || {
        console::thread_consume_messages(console_receiver, threads_count as usize)
    });

    let local_console_sender = console_sender.clone();
    let local_results_accumulator = results_accumulator.clone();
    thread::spawn(move || {
        candidates::candidate_receiver(
            candidates_receiver,
            local_cache_args,
            local_results_accumulator.clone(),
            local_console_sender,
        )
    });

    let combinations: Vec<Vec<BruteForceCryptor>> =
        combinator::combine_elements(decr.len().try_into().unwrap(), steps)
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|c| decr.clone().into_iter().nth(c.into()).unwrap())
                    .collect()
            })
            .collect();
    let mut filtered_combinations = Vec::new();
    for c in combinations.into_iter() {
        if !skip_combination(c.clone()) {
            filtered_combinations.push(c.clone());
        }
    }

    thread_system::start(
        threads_count as usize,
        filtered_combinations,
        clues,
        vec![str],
        cache_args,
        candidates_sender,
        console_sender,
    );

    eprintln!("Result: {:?}", results_accumulator.lock().unwrap());
}

pub fn skip_combination(combination: Vec<BruteForceCryptor>) -> bool {
    let not_first = vec![Some(&BruteForceCryptor::Join)];
    if not_first.contains(&combination.first()) {
        return true;
    }

    let not_last = vec![
        Some(&BruteForceCryptor::Join),
        Some(&BruteForceCryptor::Cut),
    ];
    if not_last.contains(&combination.last()) {
        return true;
    }
    let mut last = None;

    for next in combination.into_iter() {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn skip_combination_works() {
        assert_eq!(
            skip_combination(vec![
                models::BruteForceCryptor::Permute(models::BruteForcePermuteArgs {
                    max_permutations: 4
                },),
                models::BruteForceCryptor::AtBash
            ]),
            false
        )
    }
}
