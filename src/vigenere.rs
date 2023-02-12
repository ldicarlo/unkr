use crate::char_utils;

pub fn get_max_seed(_: usize) -> u64 {
    3
}

pub fn encrypt(str: Vec<String>, seed: u64) -> Vec<String> {
    vigenere(str, seed, true)
}

pub fn decrypt(str: Vec<String>, seed: u64) -> Vec<String> {
    vigenere(str, seed, false)
}

pub fn vigenere(str: Vec<String>, seed: u64, order: bool) -> Vec<String> {
    let words = vec![
        "PALIMPSEST".to_string(),
        "ABSCISSA".to_string(),
        "KRYPTOS".to_string(),
    ];
    let key_idx: usize = seed.try_into().unwrap();
    encrypt_from_key(str, words[key_idx].clone(), order)
}

pub fn encrypt_from_key(strs: Vec<String>, key: String, order: bool) -> Vec<String> {
    strs.iter()
        .map(|str| encrypt_one_from_key(str.chars().collect(), key.chars().collect(), order))
        .collect()
}

pub fn encrypt_one_from_key(str: Vec<char>, key: Vec<char>, order: bool) -> String {
    let mut result: Vec<char> = Vec::new();
    for (idx, c) in str.iter().enumerate() {
        result.push(
            char_utils::char_position(key[(idx % key.len())])
                .map(|pos| char_utils::char_mod(*c, pos, order))
                .unwrap_or(*c),
        );
    }
    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["LXFOPVEFRNHR".to_string(), "LXFOPVEFRNHR".to_string()],
            super::encrypt_from_key(
                vec!["ATTACKATDAWN".to_string(), "ATTACKATDAWN".to_string()],
                "LEMON".to_string(),
                true
            ),
        );
    }
}
