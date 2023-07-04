use super::cut;
use super::models;
use super::parser;
use super::vigenere;
use crate::atbash;
use crate::caesar;
use crate::colorize;
use crate::enigma;
use crate::indexcrypt;
use crate::join;
use crate::permute;
use crate::reverse;
use crate::swap;
use crate::transpose;

pub fn decrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    decryptors
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(strs, |acc, args| match args {
            models::CLICryptor::Vigenere(args) => vigenere::decrypt(acc, args),
            models::CLICryptor::Cut(_) => cut::decrypt(acc),
            models::CLICryptor::Caesar(number) => caesar::decrypt(acc, number),
            models::CLICryptor::Transpose(number) => transpose::decrypt(acc, number),
            models::CLICryptor::AtBash => atbash::decrypt(acc),
            models::CLICryptor::Reverse => reverse::decrypt(acc),
            models::CLICryptor::Swap(order) => swap::decrypt(acc, order),
            models::CLICryptor::Join => join::decrypt(acc),
            models::CLICryptor::Colors(letters) => colorize::colorize_letters(acc, letters),
            models::CLICryptor::IndexCrypt(letters) => indexcrypt::decrypt(acc, letters),
            models::CLICryptor::Permute(permutations) => permute::cli_decrypt(acc, permutations),
            models::CLICryptor::Enigma(enigma_args) => enigma::decrypt(acc, enigma_args),
        })
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn print_decrypt(str: Vec<String>, decryptors: Vec<String>) {
    decrypt(str, decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}
