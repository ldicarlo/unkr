use std::collections::HashMap;

use super::decryptors;
pub struct AtBash {}

impl decryptors::Decryptor for AtBash {
    fn get_max_seed(&self) -> u64 {
        1
    }

    fn decrypt(&self, str: String, _seed: u64) -> String {
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
    }
}

#[cfg(test)]
mod tests {
    use crate::decryptors::Decryptor;

    #[test]
    fn it_works() {
        let decryptor = super::AtBash {};
        assert_eq!(decryptor.decrypt("HELLO45?".to_string(), 1), "SVOOL45?");
    }
}
