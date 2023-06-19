use enigma::EnigmaArgs;
use models::BruteForceCryptor;

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
mod encrypt;
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

pub fn print_encrypt(strs: Vec<String>, decryptors: Vec<String>) {
    encrypt::print_encrypt(strs, decryptors)
}

pub fn print_decrypt(str: Vec<String>, decryptors: Vec<String>) {
    decrypt::print_decrypt(str, decryptors)
}

pub fn brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors: Vec<String>,
    threads_count: u8,
    cache_name: String,
) {
    brute_force::brute_force_decrypt(str, clues, steps, decryptors, threads_count, cache_name)
}

pub fn get_decryptors() -> Vec<BruteForceCryptor> {
    cryptors::get_decryptors()
}

pub fn brute_force_unique_combination(
    str: String,
    clues: Vec<String>,
    decryptors: Vec<String>,
    threads_count: u8,
    cache_name: String,
) {
    brute_force::brute_force_unique_combination(str, clues, decryptors, threads_count, cache_name)
}

pub fn print_combine_elements(elements_count: u8, picks: u8) {
    combinator::print_combine_elements(elements_count, picks)
}

pub fn read_bruteforce_parameters(str: String) -> BruteForceCryptor {
    parser::read_bruteforce_parameters(str)
}

pub fn fuzz_from(str: String, len_max: usize, base: usize, rules: Vec<String>) {
    fuzzer::fuzz_from(str, len_max, base, rules)
}
