use std::cmp::min;

use super::models;

pub fn encrypt_from_args(
    strs: Vec<String>,
    models::NumberArgs { number }: models::NumberArgs,
) -> Vec<String> {
    encrypt(strs, number)
}

pub fn encrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    strs.iter()
        .map(|str| str.split_at(min(str.len(), seed.try_into().unwrap())))
        .flat_map(|(a, b)| vec![a.to_string(), b.to_string()])
        .collect()
}

pub fn decrypt(strs: Vec<String>, _: u64) -> Vec<String> {
    vec![strs.join("")]
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["ABCDEF".to_string()],
            super::decrypt(vec!["ABC".to_string(), "DEF".to_string()], 1),
        );
    }
    #[test]
    fn it_works_2() {
        assert_eq!(
            vec!["ABCD", "EF"],
            super::encrypt(vec!["ABCDEF".to_string()], 4),
        );
    }
}
