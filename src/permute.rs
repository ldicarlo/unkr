use crate::{
    char_utils::{self, get_alphabet},
    vigenere::encrypt_from_key,
};

pub fn encrypt(strs: Vec<String>, alphabet: String) -> Vec<String> {
    strs
}

pub fn decrypt(strs: Vec<String>, alphabet: String) -> Vec<String> {
    encrypt_from_key(
        strs,
        get_alphabet().into_iter().collect(),
        true,
        alphabet.clone().chars().collect(),
    )
}

pub fn seed(_: usize) -> u64 {
    1
    //    403291461126605635584000000
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            super::decrypt_string("KRYPTOS".to_string(), "KRYPTOS".to_string()),
            "ABCDEF".to_string()
        );
    }
}
