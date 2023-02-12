#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum CryptorsArgs {
    Vigenere(VigenereArgs),
    Cut(SimpleArgs),
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub enum CryptorType {
    Vigenere,
    Cut,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct VigenereArgs {
    pub key: String,
    pub alphabet: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct SimpleArgs {
    pub number: u64,
}

pub enum Cryptors {
    Vigenere,
    Cut
}

pub struct CryptorPayload {
    name: String,
    seed: Box<dyn Fn(usize) -> u64>,
    encrypt: Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
    decrypt: Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
}
