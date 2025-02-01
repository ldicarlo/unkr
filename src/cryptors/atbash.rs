use core::option::Option::None;
use std::collections::HashMap;

use crate::models;

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![models::BruteForceCryptor::AtBash]
}

pub fn decrypt(strs: Vec<String>) -> Vec<String> {
    let letters: HashMap<char, char> = vec![
        ('A', 'Z'),
        ('B', 'Y'),
        ('C', 'X'),
        ('D', 'W'),
        ('E', 'V'),
        ('F', 'U'),
        ('G', 'T'),
        ('H', 'S'),
        ('I', 'R'),
        ('J', 'Q'),
        ('K', 'P'),
        ('L', 'O'),
        ('M', 'N'),
        ('N', 'M'),
        ('O', 'L'),
        ('P', 'K'),
        ('Q', 'J'),
        ('R', 'I'),
        ('S', 'H'),
        ('T', 'G'),
        ('U', 'F'),
        ('V', 'E'),
        ('W', 'D'),
        ('X', 'C'),
        ('Y', 'B'),
        ('Z', 'A'),
    ]
    .into_iter()
    .collect();
    strs.iter()
        .map(|str| {
            str.chars()
                .into_iter()
                .map(|c| {
                    let res = c.clone();
                    let maybe_char = letters.get(&c);
                    match maybe_char {
                        Some(found) => *found,
                        None => res,
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            super::decrypt(vec!["HELLO45?".to_string()]),
            vec!["SVOOL45?"]
        );
    }
    #[test]
    fn it_works_2() {
        assert_eq!(super::decrypt(vec!["BERLIN".to_string()]), vec!["YVIORM"]);
    }
}
