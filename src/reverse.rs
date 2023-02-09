pub fn decrypt(str: String, _: u64) -> String {
    str.chars().rev().collect()
}
