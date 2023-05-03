use super::cut;
use super::models;
use super::parser;
use super::vigenere;
use crate::atbash;
use crate::caesar;
use crate::join;
use crate::models::SwapArgs;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::transpose;

pub fn encrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    decryptors
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(strs, |acc, args| match args {
            models::CryptorArgs::Vigenere(args) => vigenere::encrypt(acc, args),
            models::CryptorArgs::Cut(args) => cut::encrypt(acc, args),
            models::CryptorArgs::Caesar(number) => caesar::encrypt(acc, number),
            models::CryptorArgs::Transpose(number) => transpose::decrypt(acc, number),
            models::CryptorArgs::AtBash => atbash::decrypt(acc),
            models::CryptorArgs::Reverse => reverse::decrypt(acc),
            models::CryptorArgs::Swap(order) => swap::encrypt(acc, order),
            models::CryptorArgs::Join => join::decrypt(acc),
            models::CryptorArgs::Colors(_) => acc,
            models::CryptorArgs::IndexCrypt(_) => acc,
            models::CryptorArgs::Permute(permutations) => permute::decrypt(acc, permutations),
        })
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn print_encrypt(str: String, decryptors: Vec<String>) {
    encrypt(vec![str], decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}
