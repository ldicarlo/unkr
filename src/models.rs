#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum CryptorArgs {
    Vigenere(VigenereArgs),
    Cut(SimpleArgs),
    Caesar(SimpleArgs),
    Transpose(SimpleArgs),
    AtBash,
    Reverse,
    Swap(SwapArgs),
    Join,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub enum CryptorTypeWithArgs {
    Vigenere,
    Cut,
    Caesar,
    Transpose,
    Swap,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct VigenereArgs {
    pub key: String,
    pub alphabet: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct SwapArgs {
    pub order: Vec<usize>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct SimpleArgs {
    pub number: u64,
}

pub enum Cryptors {
    Vigenere,
    Cut,
}

pub struct CryptorPayload {
    name: String,
    seed: Box<dyn Fn(usize) -> u64>,
    encrypt: Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
    decrypt: Box<dyn Fn(Vec<String>, u64) -> Vec<String>>,
}
