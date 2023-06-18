use enigma::EnigmaArgs;

mod atbash;
mod base;
mod brute_force;
mod brute_force_state;
mod cache;
mod caesar;
mod candidates;
mod char_utils;
mod colorize;
mod combinator;
mod console;
mod cryptors;
mod cut;
mod decrypt;
mod enigma;
mod fuzzer;
mod indexcrypt;
mod join;
mod models;
mod parser;
mod permute;
mod reverse;
mod swap;
mod thread_system;
mod transpose;
mod vigenere;

pub fn fuzz_next_string_ruled(
    str: &String,
    len_max: usize,
    base: usize,
    unique_letters_constraint: bool,
    pair_length_constraint: bool,
    sorted_by_pair_constraint: bool,
) -> Option<String> {
    fuzzer::fuzz_next_string_ruled(
        str,
        len_max,
        base,
        unique_letters_constraint,
        pair_length_constraint,
        sorted_by_pair_constraint,
    )
}

pub fn enigma_next(enigma_args: EnigmaArgs) -> Option<EnigmaArgs> {
    enigma::next(enigma_args)
}

pub fn enigma_init() -> EnigmaArgs {
    enigma::init()
}

pub fn enigma_encrypt(strs: Vec<String>, enigma_args: EnigmaArgs) -> Vec<String> {
    enigma::encrypt(strs, enigma_args)
}
