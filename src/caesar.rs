pub fn get_max_seed(_: usize) -> u64 {
    26
}

pub fn decrypt(str: Vec<String>, seed: u64) -> Vec<String> {
    str.chars()
        .into_iter()
        .map(|c| char_mod(c, seed.try_into().unwrap()))
        .collect()
}

pub fn encrypt(str: Vec<String>, seed: u64) -> Vec<String> {
    let size = get_max_seed(str.clone().len());
    decrypt(str, size - seed)
}

fn char_mod(c: char, number: usize) -> char {
    let alphabet = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    alphabet
        .binary_search(&c)
        .ok()
        .and_then(|index| alphabet.get((index + number) % 26))
        .map(|ch| *ch)
        .unwrap_or(c)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(char_mod('A', 5), 'F')
    }
    #[test]
    fn it_works_2() {
        assert_eq!(decrypt("MYNAMEISCAESAR".to_string(), 10), "WIXKWOSCMKOCKB")
    }
    #[test]
    fn it_works_3() {
        assert_eq!(decrypt("YVIORM".to_string(), 1), "ZWJPSN")
    }

    #[test]
    fn it_works_4() {
        assert_eq!(decrypt(encrypt("YVIORM".to_string(), 10), 10), "YVIORM")
    }
}
