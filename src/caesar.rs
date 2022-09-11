use super::decryptors;
pub struct Caesar {}

impl decryptors::Decryptor for Caesar {
    fn get_max_seed(&self) -> u64 {
        26
    }

    fn decrypt(&self, str: String, seed: u64) -> String {
        str.chars()
            .into_iter()
            .map(|c| self.char_mod(c, seed.try_into().unwrap()))
            .collect()
    }
}

impl Caesar {
    fn char_mod(&self, c: char, number: usize) -> char {
        let alphabet = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        alphabet
            .binary_search(&c)
            .ok()
            .and_then(|index| alphabet.get((index + number) % 26))
            .map(|ch| *ch)
            .unwrap_or(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::decryptors::Decryptor;

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Caesar {}.char_mod('A', 5), 'F')
    }
    #[test]
    fn it_works_2() {
        assert_eq!(
            Caesar {}.decrypt("MYNAMEISCAESAR".to_string(), 10),
            "WIXKWOSCMKOCKB"
        )
    }
}
