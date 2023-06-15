use super::combinator;
use crate::atbash;
use crate::cache;
use crate::caesar;
use crate::candidates;
use crate::console;
use crate::cut;
use crate::enigma;
use crate::join;
use crate::models;
use crate::models::BruteForceCryptor;
use crate::parser;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::thread_system;
use crate::transpose;
use crate::vigenere;
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
            true
        )
    }
}
