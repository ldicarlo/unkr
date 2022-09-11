pub trait Decryptor {
    fn get_max_seed(&self) -> u64;
    fn decrypt(&self, str: String, seed: u64) -> String;
}
