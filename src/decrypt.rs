use crate::atbash;
use crate::caesar;
use crate::colorize;
use crate::join;
use crate::models::ColorsArgs;
use crate::models::SimpleArgs;
use crate::models::SwapArgs;
use crate::reverse;
use crate::swap;
use crate::transpose;

use super::cut;
use super::models;
use super::parser;
use super::vigenere;

pub fn decrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    decryptors
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(strs, |acc, args| match args {
            models::CryptorArgs::Vigenere(args) => vigenere::decrypt_from_args(acc, args),
            models::CryptorArgs::Cut(args) => cut::encrypt_from_args(acc, args),
            models::CryptorArgs::Caesar(SimpleArgs { number }) => caesar::decrypt(acc, number),
            models::CryptorArgs::Transpose(SimpleArgs { number }) => {
                transpose::decrypt(acc, number)
            }
            models::CryptorArgs::AtBash => atbash::decrypt_from_args(acc),
            models::CryptorArgs::Reverse => reverse::decrypt_from_args(acc),
            models::CryptorArgs::Swap(SwapArgs { order }) => swap::decrypt_from_args(acc, order),
            models::CryptorArgs::Join => join::join(acc),
            models::CryptorArgs::Colors(ColorsArgs { letters }) => {
                colorize::colorize_letters(acc, letters)
            }
        })
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}
