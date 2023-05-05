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
            models::Cryptor::Vigenere(args) => vigenere::decrypt(acc, args),
            models::Cryptor::Cut(_) => cut::decrypt(acc),
            models::Cryptor::Caesar(number) => caesar::decrypt(acc, number),
            models::Cryptor::Transpose(number) => transpose::decrypt(acc, number),
            models::Cryptor::AtBash => atbash::decrypt(acc),
            models::Cryptor::Reverse => reverse::decrypt(acc),
            models::Cryptor::Swap(order) => swap::decrypt(acc, order),
            models::Cryptor::Join => join::decrypt(acc),
            models::Cryptor::Colors(letters) => colorize::colorize_letters(acc, letters),
            models::Cryptor::IndexCrypt(StringArgs { letters }) => {
                indexcrypt::decrypt(acc, letters)
            }
            models::Cryptor::Permute(permutations) => permute::decrypt(acc, permutations),
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
