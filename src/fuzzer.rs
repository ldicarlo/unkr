pub fn fuzz_next(str: String,size_max: u8) -> String {
    str
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            super::fuzz_next("KRYPTOR".to_string(),7),
            "KRYPTOS".to_string()
        );
    }
}
