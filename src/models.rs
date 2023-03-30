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
    Permute(StringArgs),
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
pub struct VigenereArgs {
    pub key: String,
    pub alphabet: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct SwapArgs {
    pub order: Vec<usize>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct StringArgs {
    pub letters: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct NumberArgs {
    pub number: u64,
}
