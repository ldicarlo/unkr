pub fn get_max_seed(text_length: u8) -> u64 {
    text_length.into()
}

pub fn encrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    strs.iter()
        .map(|str| str.split_at(seed.try_into().unwrap()))
        .flat_map(|(a, b)| vec![a.to_string(), b.to_string()])
        .collect()
}

pub fn decrypt(strs: Vec<String>, _: u64) -> Vec<String> {
    strs
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["ABC".to_string(), "DEF".to_string()],
            super::decrypt(vec!["ABC".to_string(), "DEF".to_string()], 1),
        );
    }
}
