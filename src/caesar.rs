use super::char_utils::char_mod;

pub fn get_max_seed(_: usize) -> u64 {
    26
}

pub fn decrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    strs.iter()
        .map(|str| {
            str.chars()
                .into_iter()
                .map(|c| char_mod(c, seed.try_into().unwrap(), true))
                .collect()
        })
        .collect()
}

pub fn encrypt(str: Vec<String>, seed: u64) -> Vec<String> {
    let size = get_max_seed(str.clone().len());
    decrypt(str, size - seed)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(char_mod('A', 5, true), 'F')
    }
    #[test]
    fn it_works_2() {
        assert_eq!(
            decrypt(vec!["MYNAMEISCAESAR".to_string()], 10),
            vec!["WIXKWOSCMKOCKB"]
        )
    }
    #[test]
    fn it_works_3() {
        assert_eq!(decrypt(vec!["YVIORM".to_string()], 1), vec!["ZWJPSN"])
    }

    #[test]
    fn it_works_4() {
        assert_eq!(
            decrypt(encrypt(vec!["YVIORM".to_string()], 10), 10),
            vec!["YVIORM"]
        )
    }
}
