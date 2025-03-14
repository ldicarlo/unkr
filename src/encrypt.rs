use super::models;
use super::parser;
use crate::colorize;
use crate::cryptors::atbash;
use crate::cryptors::caesar;
use crate::cryptors::cut;
use crate::cryptors::enigma;
use crate::cryptors::indexcrypt;
use crate::cryptors::join;
use crate::cryptors::permute;
use crate::cryptors::reverse;
use crate::cryptors::swap;
use crate::cryptors::transpose;
use crate::cryptors::vigenere;

pub fn encrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    decryptors
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(strs, |acc, args| match args {
            models::CLICryptor::Vigenere(args) => vigenere::encrypt(acc, args),
            models::CLICryptor::Cut(args) => cut::encrypt(acc, args),
            models::CLICryptor::Caesar(number) => caesar::encrypt(acc, number),
            models::CLICryptor::Transpose(number) => transpose::encrypt(acc, number),
            models::CLICryptor::AtBash => atbash::decrypt(acc),
            models::CLICryptor::Reverse => reverse::decrypt(acc),
            models::CLICryptor::Swap(order) => swap::encrypt(acc, order),
            models::CLICryptor::Join => join::decrypt(acc),
            models::CLICryptor::Colors(letters) => colorize::colorize_letters(acc, letters),
            models::CLICryptor::IndexCrypt(string) => indexcrypt::encrypt(acc, string),
            models::CLICryptor::Permute(permutations) => permute::cli_decrypt(acc, permutations),
            models::CLICryptor::Enigma(enigma_args) => enigma::encrypt(acc, enigma_args),
        })
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn print_encrypt(strs: Vec<String>, decryptors: Vec<String>) {
    encrypt(strs, decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}
