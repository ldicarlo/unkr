pub trait Decryptor: Clone {
  fn decrypt(str: String, seed: u64) -> String;
}

pub trait HasSeed: Clone {
  fn get_max_seed() -> u64;
}
