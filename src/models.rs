#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum Cryptor {
    Vigenere(VigenereArgs),
    Cut(NumberArgs),
    Caesar(NumberArgs),
    Transpose(NumberArgs),
    AtBash,
    Reverse,
    Swap(SwapArgs),
    Join,
    Colors(StringArgs),
    IndexCrypt(StringArgs),
    Permute(PermuteArgs),
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, Copy)]
pub enum CryptorTypeWithArgs {
    Vigenere,
    Cut,
    Caesar,
    Transpose,
    Swap,
    Colors,
    IndexCrypt,
    Permute,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum BruteForceCryptor {
    Vigenere(BruteForceVigenereArgs),
    Cut,
    Caesar,
    Transpose,
    AtBash,
    Reverse,
    Swap,
    Join,
    IndexCrypt,
    Permute(BruteForcePermuteArgs),
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, Copy)]
pub enum CryptorTypeWithBruteForceArgs {
    Vigenere,
    Permute,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct VigenereArgs {
    pub key: String,
    pub alphabet: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, Copy)]
pub struct BruteForceVigenereArgs {
    pub alphabet_depth: usize,
    pub key_depth: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct SwapArgs {
    pub order: Vec<u8>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct PermuteArgs {
    pub permutations: Vec<(char, char)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct BruteForcePermuteArgs {
    pub max_permutations: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct StringArgs {
    pub letters: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, Copy)]
pub struct NumberArgs {
    pub number: usize,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct CacheArgs {
    pub md5_string: String,
    pub md5_clues: String,
}

pub struct HitLine {
    pub args: String,
    pub result: String,
}

pub struct DoneLine {
    pub args: String,
    pub combinations: String,
}
