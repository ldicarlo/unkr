use super::atbash;
use super::caesar;
use super::cut;
use super::join;
use super::reverse;
use super::transpose;
use super::vigenere;

fn get_decryptors() -> Vec<(
    String,
    Box<dyn Fn(usize) -> u64>,
    Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
    Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
)> {
    vec![
        (
            "atbash".to_string(),
            Box::new(self::get_max_seed_is_one),
            Box::new(atbash::decrypt),
            Box::new(atbash::decrypt),
        ),
        (
            "caesar".to_string(),
            Box::new(caesar::get_max_seed),
            Box::new(caesar::decrypt),
            Box::new(caesar::encrypt),
        ),
        (
            "reverse".to_string(),
            Box::new(self::get_max_seed_is_one),
            Box::new(reverse::decrypt),
            Box::new(reverse::decrypt),
        ),
        (
            "transpose".to_string(),
            Box::new(self::get_max_seed_is_length),
            Box::new(transpose::decrypt),
            Box::new(transpose::decrypt),
        ),
        (
            "vigenere".to_string(),
            Box::new(vigenere::get_max_seed),
            Box::new(vigenere::decrypt),
            Box::new(vigenere::encrypt),
        ),
        (
            "cut".to_string(),
            Box::new(self::get_max_seed_is_length),
            Box::new(cut::decrypt),
            Box::new(cut::encrypt),
        ),
        (
            "join".to_string(),
            Box::new(self::get_max_seed_is_one),
            Box::new(join::join_seed),
            Box::new(join::join_seed),
        ),
    ]
}

pub fn filter_decryptors(
    decryptors_filtered: Vec<String>,
) -> Vec<(
    String,
    Box<dyn Fn(usize) -> u64>,
    Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
    Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
)> {
    if decryptors_filtered.is_empty() {
        get_decryptors()
    } else {
        get_decryptors()
            .into_iter()
            .filter(|(decryptor, _, _, _)| decryptors_filtered.contains(decryptor))
            .collect()
    }
}

pub fn get_max_seed_is_length(text_length: usize) -> u64 {
    text_length.try_into().unwrap()
}

pub fn get_max_seed_is_one(_: usize) -> u64 {
    1
}
