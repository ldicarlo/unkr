use super::cut;
use super::models;
use super::parser;
use super::vigenere;
use crate::atbash;
use crate::caesar;
use crate::colorize;
use crate::indexcrypt;
use crate::join;
use crate::models::StringArgs;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::transpose;

pub fn decrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    decryptors
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(strs, |acc, args| match args {
            models::CryptorArgs::Vigenere(args) => vigenere::decrypt(acc, args),
            models::CryptorArgs::Cut(_) => cut::decrypt(acc),
            models::CryptorArgs::Caesar(number) => caesar::decrypt(acc, number),
            models::CryptorArgs::Transpose(number) => transpose::decrypt(acc, number),
            models::CryptorArgs::AtBash => atbash::decrypt(acc),
            models::CryptorArgs::Reverse => reverse::decrypt(acc),
            models::CryptorArgs::Swap(order) => swap::decrypt(acc, order),
            models::CryptorArgs::Join => join::decrypt(acc),
            models::CryptorArgs::Colors(letters) => colorize::colorize_letters(acc, letters),
            models::CryptorArgs::IndexCrypt(StringArgs { letters }) => {
                indexcrypt::decrypt(acc, letters)
            }
            models::CryptorArgs::Permute(permutations) => permute::decrypt(acc, permutations),
        })
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn print_decrypt(str: String, decryptors: Vec<String>) {
    decrypt(vec![str], decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}
