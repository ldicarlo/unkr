pub fn decrypt(strs: Vec<String>) -> Vec<String> {
    strs.iter().map(|str| str.chars().rev().collect()).collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["OLLEH", "DLROW"],
            super::decrypt(vec!["HELLO".to_string(), "WORLD".to_string()]),
        );
    }
}
