use super::atbash;
use super::caesar;
use super::reverse;
use super::combinator;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn get_decryptors() -> Vec<(
    u8,
    String,
    Box<dyn Fn() -> u64>,
    Box<dyn Fn(String, u64) -> String>,
    Box<dyn Fn(String, u64) -> String>,
)> {
    vec![
        (
            1,
            "atbash".to_string(),
            Box::new(atbash::get_max_seed),
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
            Box::new(reverse::get_max_seed),
            Box::new(reverse::decrypt),
            Box::new(reverse::decrypt),
        ),
    ]
}