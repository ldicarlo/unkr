use std::collections::BTreeMap;

use crate::{char_utils, fuzzer, models};

pub fn skip_if_previous_in(args: models::BruteForcePermuteArgs) -> Vec<models::BruteForceCryptor> {
    vec![models::BruteForceCryptor::Permute(args)]
}

pub fn init() -> models::PermuteArgs {
    models::PermuteArgs {
        permutations: BTreeMap::new(),
    }
}

pub fn next(
    models::PermuteBruteForceState {
        args: models::PermuteArgs { permutations },
        brute_force_args: models::BruteForcePermuteArgs { max_permutations },
    }: models::PermuteBruteForceState,
) -> Option<models::PermuteArgs> {
    let next = fuzzer::fuzz_next_string_ruled(
        &char_utils::pairs_to_vec(permutations)
            .into_iter()
            .collect::<String>(),
        max_permutations,
        27,
        true,
        true,
        true,
    );
    next.map(|str| models::PermuteArgs {
        permutations: char_utils::vec_to_pairs(&str.chars().collect())
            .into_iter()
            .map(|(a, b)| (a as char, b as char))
            .collect(),
    })
}

pub fn decrypt(
    strs: Vec<String>,
    models::PermuteArgs { permutations }: models::PermuteArgs,
) -> Vec<String> {
    strs.iter()
        .map(|str| decrypt_string(str.clone(), permutations.clone()))
        .collect()
}

pub fn decrypt_string(str: String, permutations: BTreeMap<char, char>) -> String {
    str.chars()
        .map(|c| permutations.get(&c).unwrap_or(&c).clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::models::{BruteForcePermuteArgs, PermuteArgs, PermuteBruteForceState};

    #[test]
    fn it_works() {
        assert_eq!(
            super::decrypt_string(
                "KRYPTOS".to_string(),
                vec![('K', 'R'), ('R', 'K')].into_iter().collect()
            ),
            "RKYPTOS".to_string()
        );
    }

    #[test]
    fn next_works() {
        assert_eq!(
            super::next(PermuteBruteForceState {
                args: PermuteArgs {
                    permutations: vec![('J', 'I'), ('I', 'J')].into_iter().collect()
                },
                brute_force_args: BruteForcePermuteArgs {
                    max_permutations: 2
                }
            }),
            Some(PermuteArgs {
                permutations: vec![('J', 'K'), ('K', 'J')].into_iter().collect()
            },)
        );
    }
}
