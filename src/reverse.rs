use super::decryptors;
pub struct Caesar {}

impl decryptors::Decryptor for Caesar {
    fn get_max_seed(&self) -> u64 {
        1
    }

    fn decrypt(&self, str: String, seed: u64) -> String {
        str.chars().rev().collect()
    }
}
