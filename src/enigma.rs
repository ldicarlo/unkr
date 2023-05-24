use crate::models;

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![]
}

pub fn init() -> EnigmaArgs {
    EnigmaArgs {
        rotors: vec![('A', 0)],
        reflector: 'B',
    }
}

///
///
///  rotor A: (shifts:[+1,-3,+4 ... ], notches:[1,3])
///
///
///
pub fn next(EnigmaArgs { rotors, reflector }: EnigmaArgs) -> Option<models::PermuteArgs> {
    // let next = fuzzer::fuzz_next_string_ruled(
    //     char_utils::pairs_to_vec(permutations)
    //         .into_iter()
    //         .collect::<String>(),
    //     max_permutations,
    //     27,
    //     &vec![
    //         Box::new(fuzzer::pair_length),
    //         Box::new(fuzzer::unique_letters),
    //         Box::new(fuzzer::sorted_letters_by_pair),
    //     ],
    // );
    // next.map(|str| models::PermuteArgs {
    //     permutations: char_utils::vec_to_pairs(str.chars().collect())
    //         .into_iter()
    //         .map(|(a, b)| (a as char, b as char))
    //         .collect(),
    // })
    None
}

pub fn encrypt(strs: Vec<String>, EnigmaArgs { rotors, reflector }: EnigmaArgs) -> Vec<String> {
    strs
}

pub fn decrypt(strs: Vec<String>, EnigmaArgs { rotors, reflector }: EnigmaArgs) -> Vec<String> {
    strs
}

pub struct EnigmaArgs {
    rotors: Vec<(char, u8)>,
    reflector: char,
}
