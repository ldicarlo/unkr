use std::cmp::min;

use super::models;

pub fn encrypt(
    strs: Vec<String>,
    models::NumberArgs { number }: models::NumberArgs,
) -> Vec<String> {
    strs.iter()
        .map(|str| str.split_at(min(str.len(), number.try_into().unwrap())))
        .flat_map(|(a, b)| vec![a.to_string(), b.to_string()])
        .collect()
}

pub fn decrypt(strs: Vec<String>) -> Vec<String> {
    vec![strs.join("")]
}

#[cfg(test)]
mod tests {
    use crate::models;

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["ABCDEF".to_string()],
            super::decrypt(vec!["ABC".to_string(), "DEF".to_string()]),
        );
    }
    #[test]
    fn it_works_2() {
        assert_eq!(
            vec!["ABCD", "EF"],
            super::encrypt(vec!["ABCDEF".to_string()], models::NumberArgs { number: 4 }),
        );
    }
}
