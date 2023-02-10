pub fn decrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    strs.iter().map(|str| str.chars().rev().collect()).collect()
}
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["HELLO", "WORLD"],
            super::decrypt(vec!["HELLO".to_string(), "WORLD".to_string()], 1),
        );
    }
}
