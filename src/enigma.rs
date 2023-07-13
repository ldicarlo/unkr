use crate::{
    char_utils::{self, char_position_base},
    fuzzer, models,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![]
}

pub fn init() -> EnigmaArgs {
    EnigmaArgs {
        reflector: Reflector::B,
        l0_rotor: None,
        l_rotor: (Rotor::I, 0),
        m_rotor: (Rotor::I, 0),
        r_rotor: (Rotor::I, 0),
    }
}

/// https://cryptomuseum.com/crypto/enigma/wiring.htm
/// https://www.cryptomuseum.com/people/hamer/files/double_stepping.pdf
/// https://en.wikipedia.org/wiki/Enigma_rotor_details
/// https://piotte13.github.io/enigma-cipher/
pub fn next(enigma_args: EnigmaArgs) -> Option<EnigmaArgs> {
    let string_args = args_to_string(enigma_args.clone());
    let rotors_count = Rotor::iter().len() + 1;
    let reflector_count = Reflector::iter().len() + 1;
    let maybe_next = fuzzer::fuzz_next_string_bases(
        string_args,
        vec![
            reflector_count,
            rotors_count,
            27,
            rotors_count,
            27,
            rotors_count,
            27,
        ],
    );
    // let maybe_next = if let None = enigma_args.l0_rotor {
    //     if let Some(result) = fuzzer::fuzz_next_string_bases(
    //         string_args,
    //         vec![
    //             reflector_count,
    //             rotors_count,
    //             27,
    //             rotors_count,
    //             27,
    //             rotors_count,
    //             27,
    //         ],
    //     ) {
    //         Some(result)
    //     } else {
    //         Some(String::from("AAAAAAAAA"))
    //     }
    // } else {
    //     fuzzer::fuzz_next_string_bases(
    //         string_args,
    //         vec![
    //             reflector_count,
    //             rotors_count,
    //             27,
    //             rotors_count,
    //             27,
    //             rotors_count,
    //             27,
    //             rotors_count,
    //             27,
    //         ],
    //     )
    // };

    maybe_next.map(|next| string_to_args(next))
}

fn args_to_string(enigma_args: EnigmaArgs) -> String {
    let mut vec = Vec::new();

    vec.push(reflector_to_char(enigma_args.reflector));

    vec.push(rotor_to_char(enigma_args.l_rotor.0));

    vec.push(char_utils::get_alphabet()[enigma_args.l_rotor.1 as usize]);

    vec.push(rotor_to_char(enigma_args.m_rotor.0));

    vec.push(char_utils::get_alphabet()[enigma_args.m_rotor.1 as usize]);

    vec.push(rotor_to_char(enigma_args.r_rotor.0));

    vec.push(char_utils::get_alphabet()[enigma_args.r_rotor.1 as usize]);

    if let Some((r, i)) = enigma_args.l0_rotor {
        vec.push(rotor_to_char(r));
        vec.push(char_utils::get_alphabet()[i as usize]);
    }

    vec.iter().collect()
}

fn string_to_args(str: String) -> EnigmaArgs {
    let chars: Vec<char> = str.chars().collect();
    let reflector = char_to_reflector(chars[0])
        .expect(&format!("Could not deserialize {} to reflector", chars[0]));
    let l_rotor = (
        char_to_rotor_unwraped(chars[1]),
        char_position_base(chars[2]) as u8,
    );
    let m_rotor = (
        char_to_rotor_unwraped(chars[3]),
        char_position_base(chars[4]) as u8,
    );
    let r_rotor = (
        char_to_rotor_unwraped(chars[5]),
        char_position_base(chars[6]) as u8,
    );

    let l0_rotor: Option<(Rotor, u8)> = chars
        .clone()
        .into_iter()
        .nth(7)
        .into_iter()
        .flat_map(|c1| {
            chars
                .clone()
                .into_iter()
                .nth(8)
                .map(|c2| (char_to_rotor_unwraped(c1), char_position_base(c2) as u8))
        })
        .nth(0);

    EnigmaArgs {
        reflector,
        l0_rotor,
        l_rotor,
        m_rotor,
        r_rotor,
    }
}

fn rotor_to_char(r: Rotor) -> char {
    match r {
        Rotor::I => 'A',
        Rotor::II => 'B',
        Rotor::III => 'C',
        Rotor::IV => 'D',
        Rotor::V => 'E',
        Rotor::VI => 'F',
        Rotor::VII => 'G',
        Rotor::VIII => 'H',
    }
}

fn char_to_rotor_unwraped(c: char) -> Rotor {
    char_to_rotor(c).unwrap()
}

fn char_to_rotor(c: char) -> Option<Rotor> {
    match c {
        'A' => Some(Rotor::I),
        'B' => Some(Rotor::II),
        'C' => Some(Rotor::III),
        'D' => Some(Rotor::IV),
        'E' => Some(Rotor::V),
        'F' => Some(Rotor::VI),
        'G' => Some(Rotor::VII),
        'H' => Some(Rotor::VIII),
        _ => None,
    }
}

fn reflector_to_char(r: Reflector) -> char {
    match r {
        Reflector::B => 'A',
        Reflector::C => 'B',
    }
}

fn char_to_reflector(c: char) -> Option<Reflector> {
    match c {
        'A' => Some(Reflector::B),
        'B' => Some(Reflector::C),
        _ => None,
    }
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
    let mut result = Vec::new();
    let mut current_args = enigma_args;
    for i in str.chars() {
        let (next_char, next_rotors) = pass_through_rotors_m3(i, current_args);
        result.push(next_char);
        current_args = next_rotors;
    }

    result.iter().collect()
}

fn pass_through_rotors_m3(char: char, rotors: EnigmaArgs) -> (char, EnigmaArgs) {
    let EnigmaArgs {
        reflector,
        l0_rotor: _,
        l_rotor: (l_r, l_i),
        m_rotor: (m_r, m_i),
        r_rotor: (r_r, r_i),
    } = increment_rotors_m3(rotors);

    let new_char_1 = char_utils::char_position_base(char) as u8
        + get_rotor(r_r.clone())[(char_utils::char_position_base(char) + (r_i as usize)) % 26];
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
        EnigmaArgs {
            reflector,
            l0_rotor: None,
            l_rotor: (l_r, l_i),
            m_rotor: (m_r, m_i),
            r_rotor: (r_r, r_i),
        },
    )
}

fn increment_rotors_m3(
    EnigmaArgs {
        reflector,
        l0_rotor: _,
        l_rotor: (l_r, l_i),
        m_rotor: (m_r, m_i),
        r_rotor: (r_r, r_i),
    }: EnigmaArgs,
) -> EnigmaArgs {
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

    EnigmaArgs {
        reflector,
        l0_rotor: None,
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
        Rotor::IV => vec![10],
        Rotor::V => vec![26],
        Rotor::VI => vec![26, 13],
        Rotor::VII => vec![26, 13],
        Rotor::VIII => vec![26, 13],
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
        Rotor::IV => vec![
            4, 17, 12, 18, 11, 20, 3, 19, 16, 7, 10, 23, 5, 20, 9, 22, 23, 14, 1, 13, 16, 8, 6, 15,
            24, 2,
        ],
        Rotor::V => vec![
            21, 24, 25, 14, 2, 3, 13, 17, 12, 6, 8, 18, 1, 20, 23, 8, 10, 5, 20, 16, 22, 19, 9, 7,
            4, 11,
        ],
        Rotor::VI => vec![
            9, 14, 4, 18, 10, 15, 6, 24, 16, 7, 17, 19, 1, 20, 11, 2, 13, 19, 8, 25, 3, 16, 12, 5,
            21, 23,
        ],
        Rotor::VII => vec![
            13, 24, 7, 4, 2, 12, 22, 16, 4, 15, 8, 11, 15, 1, 6, 16, 10, 17, 3, 18, 21, 9, 14, 19,
            5, 20,
        ],
        Rotor::VIII => vec![
            5, 9, 14, 4, 15, 6, 17, 7, 20, 18, 25, 7, 3, 16, 11, 2, 10, 21, 12, 3, 19, 13, 24, 1,
            8, 22,
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
        Rotor::IV => vec![
            7, 24, 20, 18, 22, 12, 13, 6, 3, 23, 10, 4, 11, 3, 14, 15, 19, 21, 9, 25, 16, 8, 2, 17,
            10, 6,
        ],
        Rotor::V => vec![
            16, 1, 22, 8, 19, 17, 24, 6, 23, 10, 15, 3, 6, 25, 7, 20, 4, 12, 18, 13, 14, 5, 21, 18,
            9, 2,
        ],
        Rotor::VI => vec![
            18, 9, 21, 13, 7, 2, 22, 6, 14, 17, 7, 10, 20, 25, 16, 12, 19, 24, 1, 5, 11, 8, 3, 23,
            10, 15,
        ],
        Rotor::VII => vec![
            16, 11, 4, 21, 17, 10, 24, 22, 9, 19, 12, 8, 22, 13, 25, 5, 7, 14, 18, 6, 20, 23, 15,
            10, 11, 2,
        ],
        Rotor::VIII => vec![
            16, 8, 6, 10, 14, 21, 18, 22, 13, 1, 17, 20, 5, 7, 19, 23, 12, 24, 19, 11, 2, 4, 23, 9,
            25, 15,
        ],
    }
}

fn get_reflector(r: Reflector) -> Vec<u8> {
    match r {
        Reflector::B => vec![
            24, 16, 18, 4, 12, 13, 5, 22, 7, 14, 3, 21, 2, 23, 24, 19, 14, 10, 13, 6, 8, 1, 25, 12,
            2, 20,
        ],
        Reflector::C => vec![
            5, 20, 13, 6, 4, 21, 8, 17, 22, 20, 7, 14, 11, 9, 18, 13, 3, 19, 2, 23, 24, 6, 17, 15,
            9, 12,
        ],
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct EnigmaArgs {
    pub reflector: Reflector,
    pub l0_rotor: Option<(Rotor, u8)>,
    pub l_rotor: (Rotor, u8),
    pub m_rotor: (Rotor, u8),
    pub r_rotor: (Rotor, u8),
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, PartialOrd, Ord, EnumIter,
)]
pub enum Rotor {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone, PartialOrd, Ord, EnumIter,
)]
pub enum Reflector {
    //   A,
    B,
    C,
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
        offsets.push((char_utils::char_position_base(c) + 26 - i) % 26);
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
        offsets[char_utils::char_position_base(c)] =
            (26 + i - char_utils::char_position_base(c)) % 26;
    }
    println!("{}::{} =>\tvec!{:?},", prefix, key, offsets);
}

#[cfg(test)]
mod tests {
    use super::args_to_string;
    use crate::enigma::{EnigmaArgs, Reflector, Rotor};
    use strum::IntoEnumIterator;

    #[test]
    fn increment_1() {
        assert_eq!(
            EnigmaArgs {
                reflector: Reflector::B,
                l0_rotor: None,
                l_rotor: (Rotor::III, 0),
                m_rotor: (Rotor::II, 1),
                r_rotor: (Rotor::I, 17)
            },
            super::increment_rotors_m3(EnigmaArgs {
                l0_rotor: None,
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
            EnigmaArgs {
                reflector: Reflector::B,
                l0_rotor: None,
                l_rotor: (Rotor::I, 1),
                m_rotor: (Rotor::II, 6),
                r_rotor: (Rotor::III, 24)
            },
            super::increment_rotors_m3(super::increment_rotors_m3(EnigmaArgs {
                reflector: Reflector::B,
                l0_rotor: None,
                l_rotor: (Rotor::I, 0),
                m_rotor: (Rotor::II, 4),
                r_rotor: (Rotor::III, 22)
            })),
        );
    }

    // #[test]
    fn _display_rotors() {
        println!("--------- ROTORS --------");
        super::_print_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::_print_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::_print_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");

        super::_print_rotor("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB");
        super::_print_rotor("V", "VZBRGITYUPSDNHLXAWMJQOFECK");
        super::_print_rotor("VI", "JPGVOUMFYQBENHZRDKASXLICTW");
        super::_print_rotor("VII", "NZJHGRCXMYSWBOUFAIVLPEKQDT");
        super::_print_rotor("VIII", "FKQHTLXOCBJSPDZRAMEWNIUYGV");

        println!("------ REFLECTORS -------");
        super::_print_reflector("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT");
        super::_print_reflector("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL");
        println!("---- REVERSED ROTORS ----");
        super::_print_reverse_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::_print_reverse_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::_print_reverse_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");
        super::_print_reverse_rotor("IV", "ESOVPZJAYQUIRHXLNFTGKDCMWB");
        super::_print_reverse_rotor("V", "VZBRGITYUPSDNHLXAWMJQOFECK");
        super::_print_reverse_rotor("VI", "JPGVOUMFYQBENHZRDKASXLICTW");
        super::_print_reverse_rotor("VII", "NZJHGRCXMYSWBOUFAIVLPEKQDT");
        super::_print_reverse_rotor("VIII", "FKQHTLXOCBJSPDZRAMEWNIUYGV");
    }

    #[test]
    fn m3_works_1() {
        assert_eq!(
            String::from("ILBDARKFH"),
            super::encrypt_string(
                String::from("HELLOTEST"),
                super::EnigmaArgs {
                    reflector: Reflector::B,
                    l0_rotor: None,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 0)
                }
            )
        );
    }

    #[test]
    fn m3_works_2() {
        assert_eq!(
            String::from("BJE"),
            super::encrypt_string(
                String::from("ABC"),
                super::EnigmaArgs {
                    reflector: Reflector::B,
                    l0_rotor: None,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 0)
                }
            )
        );
    }

    #[test]
    fn pass_through_rotors_m3_works() {
        assert_eq!(
            super::pass_through_rotors_m3(
                'N',
                EnigmaArgs {
                    reflector: Reflector::B,
                    l0_rotor: None,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 0)
                }
            ),
            (
                'Y',
                EnigmaArgs {
                    reflector: Reflector::B,
                    l0_rotor: None,
                    l_rotor: (Rotor::I, 0),
                    m_rotor: (Rotor::II, 0),
                    r_rotor: (Rotor::III, 1)
                }
            )
        );
    }
    #[test]
    fn parse_rotors() {
        let _ = Rotor::iter()
            .map(|r| {
                assert_eq!(
                    Some(r.clone()),
                    super::char_to_rotor(super::rotor_to_char(r.clone()))
                );
                r
            })
            .collect::<Vec<Rotor>>();
    }

    #[test]
    fn parse_reflectors() {
        let _ = Reflector::iter()
            .map(|r| {
                assert_eq!(
                    Some(r.clone()),
                    super::char_to_reflector(super::reflector_to_char(r.clone()))
                );
                r
            })
            .collect::<Vec<Reflector>>();
    }

    #[test]
    fn args_to_string_works() {
        assert_eq!(
            args_to_string(EnigmaArgs {
                reflector: Reflector::B, // A
                l0_rotor: None,
                l_rotor: (Rotor::I, 3),    // A, D
                m_rotor: (Rotor::II, 2),   // B, C
                r_rotor: (Rotor::III, 12), // C, M
            }),
            String::from("AADBCCM")
        );
    }

    #[test]
    fn args_to_string_works_2() {
        let input = EnigmaArgs {
            reflector: Reflector::B, // A
            l0_rotor: Some((Rotor::III, 7)),
            l_rotor: (Rotor::I, 3),    // A, D
            m_rotor: (Rotor::II, 2),   // B, C
            r_rotor: (Rotor::III, 12), // C, M
        };
        let result = String::from("AADBCCMCH");
        assert_eq!(args_to_string(input.clone()), result);

        assert_eq!(super::string_to_args(result), input);
    }

    #[test]
    fn next_works() {
        let input = EnigmaArgs {
            reflector: Reflector::B, // A
            l0_rotor: None,
            l_rotor: (Rotor::I, 3),    // A, D
            m_rotor: (Rotor::II, 2),   // B, C
            r_rotor: (Rotor::III, 12), // C, M
        };
        let expected = EnigmaArgs {
            reflector: Reflector::B, // A
            l0_rotor: None,
            l_rotor: (Rotor::I, 3),    // A, D
            m_rotor: (Rotor::II, 2),   // B, C
            r_rotor: (Rotor::III, 13), // C, M
        };

        assert_eq!(super::next(input.clone()), Some(expected));
    }

    #[test]
    fn args_to_string_works_3() {
        let input = EnigmaArgs {
            reflector: Reflector::B, // A
            l0_rotor: None,
            l_rotor: (Rotor::I, 0), // A, D
            m_rotor: (Rotor::I, 0), // B, C
            r_rotor: (Rotor::I, 0), // C, M
        };
        let result = String::from("AAAAAAA");
        assert_eq!(args_to_string(input.clone()), result);

        assert_eq!(super::string_to_args(result), input);
    }

    #[test]
    fn full_test_1() {
        assert_eq!(
            super::encrypt_string(
                String::from("HELLOTEST"),
                EnigmaArgs {
                    reflector: Reflector::B, // A
                    l0_rotor: None,
                    l_rotor: (Rotor::III, 4), // A, D
                    m_rotor: (Rotor::VI, 19), // B, C
                    r_rotor: (Rotor::IV, 15), // C, M
                }
            ),
            String::from("OVOICLLIC")
        );
    }
}
