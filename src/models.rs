use std::collections::BTreeMap;

use crate::enigma::EnigmaArgs;

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
    Enigma(EnigmaArgs),
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
    Enigma,
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
    //  IndexCrypt,
    Permute(BruteForcePermuteArgs),
    Enigma,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, Copy)]
pub enum CryptorTypeWithBruteForceArgs {
    Vigenere,
    Permute,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum BruteForceState {
    Vigenere(VigenereBruteForceState),
    Cut(NumberArgs),
    Caesar(NumberArgs),
    Transpose(NumberArgs),
    AtBash,
    Reverse,
    Swap(SwapArgs),
    Join,
    //    IndexCrypt(StringArgs),
    Permute(PermuteBruteForceState),
    Enigma(EnigmaArgs),
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct VigenereBruteForceState {
    pub brute_force_args: BruteForceVigenereArgs,
    pub args: VigenereArgs,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct PermuteBruteForceState {
    pub brute_force_args: BruteForcePermuteArgs,
    pub args: PermuteArgs,
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
    pub permutations: BTreeMap<char, char>,
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
    pub path: String,
    pub md5_string: String,
    pub md5_clues: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct HitLine {
    pub args: Vec<Cryptor>,
    pub result: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct DoneLine {
    pub combinations: String,
    pub args: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub enum FuzzerRule {
    UniqueLetters,
    EvenCount,
    SortedLettersByPair,
}
