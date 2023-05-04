#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum CryptorArgs {
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
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum BruteForceCryptorArgs {
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
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub enum CryptorTypeWithBruteForceArgs {
    Vigenere,
    Permute,
}


#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct VigenereArgs {
    pub key: String,
    pub alphabet: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct BruteForceVigenereArgs {
    pub key_depth: String,
    pub alphabet_depth: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct SwapArgs {
    pub order: Vec<usize>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct PermuteArgs {
    pub permutations: Vec<(char, char)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct BruteForcePermuteArgs {
    pub max_permutations: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct StringArgs {
    pub letters: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct NumberArgs {
    pub number: u64,
}
