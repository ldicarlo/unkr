pub fn get_max_seed() -> u64 {
    1
}

pub fn decrypt(str: String, seed: u64) -> String {
    // string length = 102
    // seed = 5
    // 102 / 5 = 20r2
str
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
          "str".to_string(), decrypt("str".to_string(), 1)
        );
    }
}