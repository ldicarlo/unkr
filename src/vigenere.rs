use crate::fuzzer;

use super::char_utils;
use super::models;

pub fn init() -> models::VigenereArgs {
    models::VigenereArgs {
        key: String::from(""),
        alphabet: String::from(""),
    }
}

pub fn next(
    models::VigenereBruteForceState {
        args: models::VigenereArgs { key, alphabet },
        brute_force_args:
            models::BruteForceVigenereArgs {
                key_depth,
                alphabet_depth,
            },
    }: models::VigenereBruteForceState,
) -> Option<models::VigenereArgs> {
    fuzzer::fuzz_next_string_ruled(&key, key_depth, 27, false, false, false)
        .map(|k| models::VigenereArgs {
            key: k,
            alphabet: alphabet.clone(),
        })
        .or_else(|| {
            fuzzer::fuzz_next_string_ruled(&alphabet, alphabet_depth, 27, false, false, false).map(
                |a| models::VigenereArgs {
                    key: String::from("A"),
                    alphabet: a,
                },
            )
        })
}

pub fn encrypt(
    strs: Vec<String>,
    models::VigenereArgs { key, alphabet }: models::VigenereArgs,
) -> Vec<String> {
    encrypt_from_key(strs, key, true, alphabet.chars().collect())
}

pub fn decrypt(
    strs: Vec<String>,
    models::VigenereArgs { key, alphabet }: models::VigenereArgs,
) -> Vec<String> {
    encrypt_from_key(strs, key, false, alphabet.chars().collect())
}

pub fn encrypt_from_key(
    strs: Vec<String>,
    key: String,
    order: bool,
    alphabet: Vec<char>,
) -> Vec<String> {
    strs.iter()
        .map(|str| {
            encrypt_one_from_key(
                str.chars().collect(),
                key.chars().collect(),
                order,
                alphabet.clone(),
            )
        })
        .collect()
}

pub fn encrypt_one_from_key(
    strs: Vec<char>,
    key: Vec<char>,
    order: bool,
    alphabet: Vec<char>,
) -> String {
    let mut result: Vec<char> = Vec::new();
    let custom_alphabet = char_utils::merge_custom_alphabet(alphabet);
    let mut vigenere_idx = 0;
    for c in strs.iter() {
        let key_letter = key[vigenere_idx % key.len()];
        let res = char_utils::char_position(*c, custom_alphabet.clone())
            .and_then(|letter_position| {
                let key_position =
                    char_utils::char_position(key_letter, custom_alphabet.clone()).unwrap();
                vigenere_idx = vigenere_idx + 1;
                custom_alphabet.get(
                    (if order {
                        26 + letter_position + key_position
                    } else {
                        // bad fix?
                        26 + 26 + letter_position - key_position
                    }) % 26,
                )
            })
            .unwrap_or(c);

        result.push(*res);
    }
    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::models;
    #[test]
    fn it_works() {
        assert_eq!(
            vec!["LXFOPVEFRNHR".to_string(), "LXFOPVEFRNHR".to_string()],
            super::encrypt(
                vec!["ATTACKATDAWN".to_string(), "ATTACKATDAWN".to_string()],
                models::VigenereArgs {
                    key: "LEMON".to_string(),
                    alphabet: "".to_string()
                }
            ),
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            vec!["HYNLPVETV".to_string()],
            super::encrypt(
                vec!["HELLOTEST".to_string(),],
                models::VigenereArgs {
                    key: "KEY".to_string(),
                    alphabet: "KEY".to_string()
                }
            ),
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(
            vec!["HELLOTEST".to_string()],
            super::decrypt(
                vec!["HYNLPVETV".to_string(),],
                models::VigenereArgs {
                    key: "KEY".to_string(),
                    alphabet: "KEY".to_string()
                }
            ),
        );
    }
}
