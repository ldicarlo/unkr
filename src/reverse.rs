pub fn get_max_seed() -> u64 {
    1
}

pub fn decrypt(str: String, _: u64) -> String {
    str.chars().rev().collect()
}
