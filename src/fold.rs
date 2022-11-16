pub fn next_seed(last_seed: u64) -> Option<u64> {
    None
}

pub fn decrypt(str: String, seed: u64) -> String {
    // string length = 12
    // seed max =
    // seed = 14
    // 14 % 12 = 2
    // 14 / 12 = 1
    // ABCDEFGHIJKL
    // divide by 1 + 1 -> 2
    //

    str
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!("str".to_string(), decrypt("str".to_string(), 1));
    }
}
