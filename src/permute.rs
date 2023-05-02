use crate::{char_utils, fuzzer, models};

pub fn init() -> models::PermuteArgs {
    models::PermuteArgs {
        permutations: vec![],
    }
}

pub fn next(args: models::PermuteArgs) -> Option<models::PermuteArgs> {
    let models::PermuteArgs { permutations } = args;
    let next = fuzzer::fuzz_next_r(
        char_utils::vec_to_string(permutations),
        5,
        vec![
            Box::new(fuzzer::pair_length),
            Box::new(fuzzer::unique_letters),
            Box::new(fuzzer::sorted_letters_by_pair),
        ],
    );
    next.map(|str| models::PermuteArgs {
        permutations: char_utils::string_to_vec(str),
    })
}

pub fn decrypt_internal(strs: Vec<String>, permutations: Vec<(char, char)>) -> Vec<String> {
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
    use crate::models::PermuteArgs;

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
            super::next(PermuteArgs {
                permutations: vec![('J', 'I')]
            }),
            Some(PermuteArgs {
                permutations: vec![('J', 'K')]
            },)
        );
    }
}
