use super::atbash;
use super::caesar;
use super::cut;
use super::reverse;
use super::transpose;
use super::vigenere;

pub fn get_decryptors() -> Vec<(
    u8,
    String,
    Box<dyn Fn(usize) -> u64>,
    Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
    Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
)> {
    vec![
        (
            1,
            "atbash".to_string(),
            Box::new(self::get_max_seed_is_one),
            Box::new(atbash::decrypt),
            Box::new(atbash::decrypt),
        ),
        (
            2,
            "caesar".to_string(),
            Box::new(caesar::get_max_seed),
            Box::new(caesar::decrypt),
            Box::new(caesar::encrypt),
        ),
        (
            3,
            "reverse".to_string(),
            Box::new(self::get_max_seed_is_one),
            Box::new(reverse::decrypt),
            Box::new(reverse::decrypt),
        ),
        (
            4,
            "transpose".to_string(),
            Box::new(self::get_max_seed_is_length),
            Box::new(transpose::decrypt),
            Box::new(transpose::decrypt),
        ),
        (
            5,
            "vigenere".to_string(),
            Box::new(vigenere::get_max_seed),
            Box::new(vigenere::decrypt),
            Box::new(vigenere::encrypt),
        ),
        (
            6,
            "cut".to_string(),
            Box::new(self::get_max_seed_is_length),
            Box::new(cut::decrypt),
            Box::new(cut::encrypt),
        ),
    ]
}

pub fn get_max_seed_is_length(text_length: usize) -> u64 {
    text_length.try_into().unwrap()
}

pub fn get_max_seed_is_one(_: usize) -> u64 {
    1
}
