use clap::Args;

use crate::{char_utils, models};

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![]
}

pub fn init() -> EnigmaArgs {
    EnigmaArgs::M3(M3_settings {
        l_rotor: (Rotor::I, 0),
        m_rotor: (Rotor::I, 0),
        r_rotor: (Rotor::I, 0),
        reflector: Reflector::B,
    })
}

/// https://cryptomuseum.com/crypto/enigma/wiring.htm
/// https://www.cryptomuseum.com/people/hamer/files/double_stepping.pdf
/// https://en.wikipedia.org/wiki/Enigma_rotor_details
///
pub fn next(enigma_args: EnigmaArgs) -> Option<EnigmaArgs> {
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
    match enigma_args {
        EnigmaArgs::M3(args) => {
            let mut result = Vec::new();
            let mut current_args = args;
            for i in str.chars() {
                let (next_char, next_rotors) = pass_through_rotors_m3(i, current_args);
                result.push(next_char);
                current_args = next_rotors;
            }

            result.iter().collect()
        }
    }
}

fn pass_through_rotors_m3(char: char, rotors: M3_settings) -> (char, M3_settings) {
    let M3_settings {
        reflector,
        l_rotor: (l_r, l_i),
        m_rotor: (m_r, m_i),
        r_rotor: (r_r, r_i),
    } = increment_rotors_m3(rotors);

    let new_char_1 = char_utils::char_position_base(char).unwrap() as u8
        + get_rotor(r_r.clone())
            [(char_utils::char_position_base(char).unwrap() + (r_i as usize)) % 26];
    println!(
        "Char1: {}",
        char_utils::get_alphabet()[(new_char_1 % 26) as usize]
    );
    let new_char_2 =
        new_char_1 + get_rotor(m_r.clone())[(new_char_1 as usize + (m_i as usize)) % 26];
    println!(
        "Char2: {}",
        char_utils::get_alphabet()[(new_char_2 % 26) as usize]
    );
    let new_char_3 =
        new_char_2 + get_rotor(l_r.clone())[(new_char_2 as usize + (l_i as usize)) % 26];
    println!(
        "Char3: {}",
        char_utils::get_alphabet()[(new_char_3 % 26) as usize]
    );
    let new_char_4 = new_char_3 + get_reflector(reflector.clone())[(new_char_3 as usize) % 26];
    println!(
        "Char4: {}",
        char_utils::get_alphabet()[(new_char_4 % 26) as usize]
    );
    let new_char_5 =
        get_reversed_rotor(l_r.clone())[(new_char_4 as usize + (l_i as usize)) % 26];
    println!(
        "Char5: {}",
        char_utils::get_alphabet()[(new_char_5 % 26) as usize]
    );
    let new_char_6 =
         get_reversed_rotor(m_r.clone())[(new_char_5 as usize + (m_i as usize)) % 26];
    println!(
        "Char6: {}",
        char_utils::get_alphabet()[(new_char_6 % 26) as usize]
    );
    let new_char_7 =
        get_reversed_rotor(r_r.clone())[(new_char_6 as usize + (r_i as usize)) % 26];
    println!(
        "Char7: {}",
        char_utils::get_alphabet()[(new_char_7 % 26) as usize]
    );
    (
        char_utils::get_alphabet()[(new_char_7 % 26) as usize],
        M3_settings {
            reflector,
            l_rotor: (l_r, l_i),
            m_rotor: (m_r, m_i),
            r_rotor: (r_r, r_i),
        },
    )
}

fn increment_rotors_m3(
    M3_settings {
        reflector,
        l_rotor: (l_r, l_i),
        m_rotor: (m_r, m_i),
        r_rotor: (r_r, r_i),
    }: M3_settings,
) -> M3_settings {
    let new_r_rotor_i = (r_i + 1) % 26;

    let r_notches = get_notches(r_r.clone());

    let m_notches = get_notches(m_r.clone());
    let new_m_rotor_i = if r_notches.contains(&r_i) || m_notches.contains(&m_i) {
        (m_i + 1) % 26
    } else {
        m_i
    };
    let l_notches = get_notches(r_r.clone());
    let new_l_rotor_i = if m_notches.contains(&m_i) || l_notches.contains(&l_i) {
        (l_i + 1) % 26
    } else {
        l_i
    };

    M3_settings {
        reflector,
        l_rotor: (l_r, new_l_rotor_i),
        m_rotor: (m_r, new_m_rotor_i),
        r_rotor: (r_r, new_r_rotor_i),
    }
}

fn get_notches(r: Rotor) -> Vec<u8> {
    match r {
        Rotor::I => vec![16],
        Rotor::II => vec![5],
        Rotor::III => vec![22],
    }
}
// let's ignore minus, and use only plus
fn get_rotor(r: Rotor) -> Vec<u8> {
    match r {
        Rotor::I => vec![
            4, 9, 10, 2, 7, 1, 23, 9, 13, 16, 3, 8, 2, 9, 10, 18, 7, 3, 0, 22, 6, 13, 5, 20, 4, 10,
        ],
        Rotor::II => vec![
            0, 8, 1, 7, 14, 3, 11, 13, 15, 18, 1, 22, 10, 6, 24, 13, 0, 15, 7, 20, 21, 3, 9, 24,
            16, 5,
        ],
        Rotor::III => vec![
            1, 2, 3, 4, 5, 6, 22, 8, 9, 10, 13, 10, 13, 0, 10, 15, 18, 5, 14, 7, 16, 17, 24, 21,
            18, 15,
        ],
    }
}

fn get_reversed_rotor(r: Rotor) -> Vec<u8> {
    match r {
        Rotor::I => vec![
            20, 22, 24, 6, 0, 3, 5, 15, 21, 25, 1, 4, 2, 10, 12, 19, 7, 23, 18, 11, 17, 8, 13, 16,
            14, 9,
        ],
        Rotor::II => vec![
            0, 9, 15, 2, 25, 22, 17, 11, 5, 1, 3, 10, 14, 19, 24, 20, 16, 6, 4, 13, 7, 23, 12, 8,
            21, 18,
        ],
        Rotor::III => vec![
            19, 0, 6, 1, 15, 2, 18, 3, 16, 4, 20, 5, 21, 13, 25, 7, 24, 8, 23, 9, 22, 11, 17, 10,
            14, 12,
        ],
    }
}

fn get_reflector(r: Reflector) -> Vec<u8> {
    match r {
        Reflector::B => vec![
            24, 16, 18, 4, 12, 13, 5, 22, 7, 14, 3, 21, 2, 23, 24, 19, 14, 10, 13, 6, 8, 1, 25, 12,
            2, 20,
        ],
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
    //   A,
    B,
}

fn print_rotor(key: &str, str: &str) {
    print_any("Rotor", key, str)
}

fn print_reflector(key: &str, str: &str) {
    print_any("Reflector", key, str)
}

fn print_any(prefix: &str, key: &str, str: &str) {
    let mut offsets = Vec::new();
    for (i, c) in str.chars().enumerate() {
        offsets.push((char_utils::char_position_base(c).unwrap() + 26 - i) % 26);
    }
    println!("{}::{} =>\tvec!{:?},", prefix, key, offsets);
}

fn print_reverse_rotor(key: &str, str: &str) {
    print_reverse("Rotor", key, str)
}

fn print_reverse_reflector(key: &str, str: &str) {
    print_reverse("Reflector", key, str)
}

fn print_reverse(prefix: &str, key: &str, str: &str) {
    let mut offsets = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    for (i, c) in str.chars().enumerate() {
        offsets[char_utils::char_position_base(c).unwrap()] = (26 + i) % 26;
    }
    println!("{}::{} =>\tvec!{:?},", prefix, key, offsets);
}

#[cfg(test)]
mod tests {
    use crate::enigma::{M3_settings, Reflector, Rotor};

    #[test]
    fn increment_1() {
        assert_eq!(
            M3_settings {
                reflector: Reflector::B,
                l_rotor: (Rotor::III, 0),
                m_rotor: (Rotor::II, 1),
                r_rotor: (Rotor::I, 17)
            },
            super::increment_rotors_m3(M3_settings {
                reflector: Reflector::B,
                l_rotor: (Rotor::III, 0),
                m_rotor: (Rotor::II, 0),
                r_rotor: (Rotor::I, 16)
            }),
        );
    }

    #[test]
    fn increment_double_step() {
        assert_eq!(
            M3_settings {
                reflector: Reflector::B,
                l_rotor: (Rotor::I, 1),
                m_rotor: (Rotor::II, 6),
                r_rotor: (Rotor::III, 24)
            },
            super::increment_rotors_m3(super::increment_rotors_m3(M3_settings {
                reflector: Reflector::B,
                l_rotor: (Rotor::I, 0),
                m_rotor: (Rotor::II, 4),
                r_rotor: (Rotor::III, 22)
            })),
        );
    }

    #[test]
    fn _display_rotors() {
        super::print_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::print_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::print_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");
        // super::print_reflector("A", "EJMZALYXVBWFCRQUONTSPIKHGD");
        super::print_reflector("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT");
        println!("---- REVERSE ----");
        super::print_reverse_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::print_reverse_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::print_reverse_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");
    }

    #[test]
    fn m3_works_1() {
        assert_eq!(
            String::from("MFNCZXHUM"),
            super::encrypt_string(
                String::from("HELLOTEST"),
                super::EnigmaArgs::M3(M3_settings {
                    reflector: Reflector::B,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 0)
                })
            )
        );
    }

    #[test]
    fn m3_works_2() {
        assert_eq!(
            String::from("FUV"),
            super::encrypt_string(
                String::from("ABC"),
                super::EnigmaArgs::M3(M3_settings {
                    reflector: Reflector::B,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 0)
                })
            )
        );
    }

    #[test]
    fn pass_through_rotors_m3_works() {
        assert_eq!(
            super::pass_through_rotors_m3(
                'N',
                super::M3_settings {
                    reflector: Reflector::B,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 0)
                }
            ),
            (
                'Y',
                super::M3_settings {
                    reflector: Reflector::B,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 1)
                }
            )
        );
    }
}
