use super::combinator;
use crate::cryptors::{atbash, enigma, join, permute, reverse, swap};
use crate::models;
use crate::models::BruteForceCryptor;
use crate::parser;
use crate::thread_system;
use std::collections::VecDeque;
use std::vec;

pub fn brute_force_unique_combination(
    str: String,
    clues: Vec<String>,
    decryptors: Vec<String>,
    threads_numbers: Vec<u8>,
    threads_count: u8,
    cache_name: String,
    pretty: bool,
    intermediate_steps: bool,
) {
    let decr: VecDeque<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();

    thread_system::start(
        str,
        threads_numbers,
        threads_count as usize,
        vec![decr],
        clues,
        pretty,
        cache_name,
        intermediate_steps,
    );
}

pub fn brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors: Vec<String>,
    threads_numbers: Vec<u8>,
    threads_count: u8,
    pretty: bool,
    cache_name: String,
) {
    let decr: Vec<models::BruteForceCryptor> = decryptors
        .iter()
        .map(|str| parser::read_bruteforce_parameters(str.to_string()))
        .collect();

    let combinations: Vec<VecDeque<BruteForceCryptor>> =
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
        str,
        threads_numbers,
        threads_count as usize,
        filtered_combinations,
        clues,
        pretty,
        cache_name,
        true,
    );
}

pub fn skip_combination(combination: VecDeque<BruteForceCryptor>) -> bool {
    let not_first = vec![Some(&BruteForceCryptor::Join)];
    if not_first.contains(&combination.front()) {
        return true;
    }

    let not_last = vec![
        Some(&BruteForceCryptor::Join),
        Some(&BruteForceCryptor::Cut),
    ];
    if not_last.contains(&combination.back()) {
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
                BruteForceCryptor::Reuse(_) => false,
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
            skip_combination(
                vec![
                    models::BruteForceCryptor::Permute(models::BruteForcePermuteArgs {
                        max_permutations: 4
                    },),
                    models::BruteForceCryptor::AtBash
                ]
                .into()
            ),
            false
        )
    }
}
