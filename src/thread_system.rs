use std::collections::{BTreeSet, HashSet};

use crate::models::{BruteForceCryptor, CacheArgs, CryptorTypeWithArgs, DoneLine};

pub fn internal_brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors_filtered: Vec<BruteForceCryptor>,
    threads: u8,
    done_cache: BTreeSet<DoneLine>,
    cache_args: CacheArgs,
    combinations: HashSet<Vec<u8>>,
) -> BTreeSet<String> {
    let mut internal_combinations = combinations.clone();

    while internal_combinations.len() > 0 {
        while let Some(next) = next() {}
    }

    BTreeSet::new()
}

fn next() -> Option<CryptorTypeWithArgs> {
    None
}
