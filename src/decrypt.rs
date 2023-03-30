use super::cut;
use super::models;
use super::parser;
use super::vigenere;
use crate::atbash;
use crate::caesar;
use crate::colorize;
use crate::indexcrypt;
use crate::join;
use crate::models::NumberArgs;
use crate::models::PermuteArgs;
use crate::models::StringArgs;
use crate::models::SwapArgs;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::transpose;

pub fn decrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    decryptors
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(strs, |acc, args| match args {
            models::CryptorArgs::Vigenere(args) => vigenere::decrypt_from_args(acc, args),
            models::CryptorArgs::Cut(args) => cut::encrypt_from_args(acc, args),
            models::CryptorArgs::Caesar(NumberArgs { number }) => caesar::decrypt(acc, number),
            models::CryptorArgs::Transpose(NumberArgs { number }) => {
                transpose::decrypt(acc, number)
            }
            models::CryptorArgs::AtBash => atbash::decrypt_from_args(acc),
            models::CryptorArgs::Reverse => reverse::decrypt_from_args(acc),
            models::CryptorArgs::Swap(SwapArgs { order }) => swap::decrypt_from_args(acc, order),
            models::CryptorArgs::Join => join::join(acc),
            models::CryptorArgs::Colors(StringArgs { letters }) => {
                colorize::colorize_letters(acc, letters)
            }
            models::CryptorArgs::IndexCrypt(StringArgs { letters }) => {
                indexcrypt::decrypt(acc, letters)
            }
            models::CryptorArgs::Permute(PermuteArgs { permutations }) => permute::decrypt_internal(acc, permutations),
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
