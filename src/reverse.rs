pub fn get_max_seed(text_length: usize) -> u64 {
    1
}

pub fn decrypt(str: String, _: u64) -> String {
    str.chars().rev().collect()
}
