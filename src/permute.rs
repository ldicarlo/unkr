use crate::{cryptors, models};

pub fn has_next(args: models::PermuteArgs) {}

pub fn next(args: models::PermuteArgs) -> models::PermuteArgs {
    args
}

pub fn decrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    let permutations = vec![vec![('K', 'R'), ('Y', 'P'), ('T', 'O')]];

    decrypt_internal(strs, permutations[seed as usize].to_vec())
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
            PermuteArgs {
                permutations: vec![('J', 'K')]
            },
        );
    }
}
