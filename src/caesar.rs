use super::decryptors;
pub struct Caesar {}

impl decryptors::Decryptor for Caesar {
    fn get_max_seed(&self) -> u64 {
        26
    }

    fn decrypt(&self, str: String, seed: u64) -> String {
        todo!()
    }
}
