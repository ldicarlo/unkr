use crate::models;

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![]
}

pub fn init() -> EnigmaArgs {
    EnigmaArgs::M3(M3_settings {
        l_rotor: (Rotor::I, 0),
        m_rotor: (Rotor::I, 0),
        r_rotor: (Rotor::I, 0),
        reflector: Reflector::A,
    })
}

/// https://cryptomuseum.com/crypto/enigma/wiring.htm
///
///
///  rotor A: (shifts:[+1,-3,+4 ... ], notches:[1,3])
///
///
///
pub fn next(enigma_args: EnigmaArgs) -> Option<models::PermuteArgs> {
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

pub fn encrypt(strs: Vec<String>, enigma_args: EnigmaArgs) -> Vec<String> {
    strs
}

pub fn decrypt(strs: Vec<String>, enigma_args: EnigmaArgs) -> Vec<String> {
    strs
}

pub fn encrypt_string(str: String, enigma_args: EnigmaArgs) -> String {
    for i in str.chars() {}

    str
}

fn pass_through_rotors(char: char, enigma_args: EnigmaArgs) -> (char, EnigmaArgs) {
    // let new_rotors = increment_rotors(rotors);
    // for rotor in rotors.clone() {
    //     pass_through_rotor(char, rotor);
    // }
    // let c = pass_through_reflector(char, reflector.clone());
    // for rotor in rotors.clone() {
    //     pass_through_rotor(char, rotor);
    // }
    // (
    //     char,
    //     EnigmaArgs {
    //         rotors: new_rotors,
    //         reflector,
    //     },
    // )
    (char, enigma_args)
}

fn pass_through_rotor(char: char, rotor: (Rotor, u8)) -> (char, (Rotor, u8)) {
    (char, rotor)
}

fn pass_through_reflector(char: char, reflector: Reflector) -> char {
    char
}

fn increment_rotors_m3(
    M3_settings {
        reflector,
        l_rotor: (l_r, l_i),
        m_rotor: (m_r, m_i),
        r_rotor: (r_r, r_i),
    }: M3_settings,
) -> M3_settings {
    let new_l_rotor = (l_i + 1) % 26;
    let new_m_rotor = (m_i + 1) % 26;
    let new_r_rotor = (r_i + 1) % 26;
    M3_settings {
        reflector,
        l_rotor: (l_r, new_l_rotor),
        m_rotor: (m_r, new_m_rotor),
        r_rotor: (r_r, new_r_rotor),
    }
}

fn get_notches(r: Rotor) -> Vec<u8> {
    match r {
        Rotor::I => vec![2],
        Rotor::II => vec![5],
        Rotor::III => vec![7],
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub enum EnigmaArgs {
    M3(M3_settings),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]

pub struct M3_settings {
    reflector: Reflector,
    l_rotor: (Rotor, u8),
    m_rotor: (Rotor, u8),
    r_rotor: (Rotor, u8),
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
