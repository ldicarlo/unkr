

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
    strs.into_iter()
        .map(|s| encrypt_string(s, enigma_args.clone()))
        .collect()
}

pub fn decrypt(strs: Vec<String>, enigma_args: EnigmaArgs) -> Vec<String> {
    strs.into_iter()
        .map(|s| encrypt_string(s, enigma_args.clone()))
        .collect()
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

    let new_char_2 =
        new_char_1 + get_rotor(m_r.clone())[(new_char_1 as usize + (m_i as usize)) % 26];
    let new_char_3 =
        new_char_2 + get_rotor(l_r.clone())[(new_char_2 as usize + (l_i as usize)) % 26];
    let new_char_4 = new_char_3 + get_reflector(reflector.clone())[(new_char_3 as usize) % 26];
    let new_char_5 =
        new_char_4 + get_reversed_rotor(l_r.clone())[(new_char_4 as usize + (l_i as usize)) % 26];
    let new_char_6 =
        new_char_5 + get_reversed_rotor(m_r.clone())[(new_char_5 as usize + (m_i as usize)) % 26];
    let new_char_7 =
        new_char_6 + get_reversed_rotor(r_r.clone())[(new_char_6 as usize + (r_i as usize)) % 26];
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
            20, 21, 22, 3, 22, 24, 25, 8, 13, 16, 17, 19, 16, 23, 24, 4, 17, 6, 0, 18, 23, 13, 17,
            19, 16, 10,
        ],
        Rotor::II => vec![
            0, 8, 13, 25, 21, 17, 11, 4, 23, 18, 19, 25, 2, 6, 10, 5, 0, 15, 12, 20, 13, 2, 16, 11,
            23, 19,
        ],
        Rotor::III => vec![
            19, 25, 4, 24, 11, 23, 12, 22, 8, 21, 10, 20, 9, 0, 11, 18, 8, 17, 5, 16, 2, 16, 21,
            13, 16, 13,
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

fn _print_rotor(key: &str, str: &str) {
    _print_any("Rotor", key, str)
}

fn _print_reflector(key: &str, str: &str) {
    _print_any("Reflector", key, str)
}

fn _print_any(prefix: &str, key: &str, str: &str) {
    let mut offsets = Vec::new();
    for (i, c) in str.chars().enumerate() {
        offsets.push((char_utils::char_position_base(c).unwrap() + 26 - i) % 26);
    }
    println!("{}::{} =>\tvec!{:?},", prefix, key, offsets);
}

fn _print_reverse_rotor(key: &str, str: &str) {
    _print_reverse("Rotor", key, str)
}

fn _print_reverse_reflector(key: &str, str: &str) {
    _print_reverse("Reflector", key, str)
}

fn _print_reverse(prefix: &str, key: &str, str: &str) {
    let mut offsets = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    for (i, c) in str.chars().enumerate() {
        offsets[char_utils::char_position_base(c).unwrap()] =
            (26 + i - char_utils::char_position_base(c).unwrap()) % 26;
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

    //    #[test]
    fn _display_rotors() {
        println!("--------- ROTORS --------");
        super::_print_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::_print_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::_print_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");

        println!("------ REFLECTORS -------");
        super::_print_reflector("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT");

        println!("---- REVERSED ROTORS ----");
        super::_print_reverse_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::_print_reverse_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::_print_reverse_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");
    }

    #[test]
    fn m3_works_1() {
        assert_eq!(
            String::from("ILBDARKFH"),
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
            String::from("BJE"),
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
