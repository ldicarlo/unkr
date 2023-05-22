use crate::{char_utils, fuzzer, models};

pub fn skip_if_previous_in(args: models::BruteForcePermuteArgs) -> Vec<models::BruteForceCryptor> {
    vec![models::BruteForceCryptor::Permute(args)]
}

pub fn init() -> models::PermuteArgs {
    models::PermuteArgs {
        permutations: vec![],
    }
}

pub fn next(
    models::PermuteArgs { permutations }: models::PermuteArgs,
    models::BruteForcePermuteArgs { max_permutations }: models::BruteForcePermuteArgs,
) -> Option<models::PermuteArgs> {
    let next = fuzzer::fuzz_next_string_ruled(
        char_utils::pairs_to_vec(permutations)
            .into_iter()
            .collect::<String>(),
        max_permutations,
        27,
        &vec![
            Box::new(fuzzer::pair_length),
            Box::new(fuzzer::unique_letters),
            Box::new(fuzzer::sorted_letters_by_pair),
        ],
    );
    next.map(|str| models::PermuteArgs {
        permutations: char_utils::vec_to_pairs(str.chars().collect())
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

pub fn decrypt_string(str: String, permutations: Vec<(char, char)>) -> String {
    str.chars()
        .map(|c| {
            permutations
                .iter()
                .find(|(a, b)| c == *a || c == *b)
                .map_or(c, |(a, b)| if c == *a { *b } else { *a })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::models::{BruteForcePermuteArgs, PermuteArgs};

    #[test]
    fn it_works() {
        assert_eq!(
            super::decrypt_string("KRYPTOS".to_string(), vec![('K', 'R')]),
            "RKYPTOS".to_string()
        );
    }

    #[test]
    fn next_works() {
        assert_eq!(
            super::next(
                PermuteArgs {
                    permutations: vec![('J', 'I')]
                },
                BruteForcePermuteArgs {
                    max_permutations: 2
                }
            ),
            Some(PermuteArgs {
                permutations: vec![('J', 'K')]
            },)
        );
    }
}
