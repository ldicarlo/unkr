use crate::models;

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![]
}

pub fn init() -> EnigmaArgs {
    EnigmaArgs {
        rotors: vec![(Rotor::I, 0)],
        reflector: Reflector::A,
    }
}

/// https://cryptomuseum.com/crypto/enigma/wiring.htm
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

pub fn encrypt_string(str: String, EnigmaArgs { rotors, reflector }: EnigmaArgs) -> String {
    for i in str.chars() {}

    str
}

fn pass_through_rotors(
    char: char,
    EnigmaArgs { rotors, reflector }: EnigmaArgs,
) -> (char, EnigmaArgs) {
    let new_rotors = increment_rotors(rotors);
    for rotor in rotors.clone() {
        pass_through_rotor(char, rotor);
    }
    let c = pass_through_reflector(char, reflector.clone());
    for rotor in rotors.clone() {
        pass_through_rotor(char, rotor);
    }
    (
        char,
        EnigmaArgs {
            rotors: new_rotors,
            reflector,
        },
    )
}

fn pass_through_rotor(char: char, rotor: (Rotor, u8)) -> (char, (Rotor, u8)) {
    (char, rotor)
}

fn pass_through_reflector(char: char, reflector: Reflector) -> char {
    char
}

fn increment_rotors(rotors: Vec<(Rotor, u8)>) -> Vec<(Rotor, u8)> {
    rotors
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct EnigmaArgs {
    reflector: Reflector,
    rotors: Vec<(Rotor, u8)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum Rotor {
    I,
    II,
    III,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum Reflector {
    A,
    B,
}

#[cfg(test)]
mod tests {
    use crate::enigma::Rotor;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![(Rotor::I, 3), (Rotor::II, 3), (Rotor::III, 3)],
            super::increment_rotors(vec![(Rotor::I, 3), (Rotor::II, 3), (Rotor::III, 3)]),
        );
    }
}
