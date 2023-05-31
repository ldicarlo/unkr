use crate::{char_utils, models};

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
    for i in str.chars() {}

    str
}

fn pass_through_rotors_m3(char: char, rotors: M3_settings) -> (char, M3_settings) {
    let  M3_settings {
      reflector,
      l_rotor: (l_r, l_i),
      m_rotor: (m_r, m_i),
      r_rotor: (r_r, r_i),
  } = increment_rotors_m3(rotors);



let new_char_1 =  get_rotor(r_r)[( char as usize + (r_i as usize) ) % 26 ];
let new_char_2 =  get_rotor(r_r)[( new_char_1 as usize + (r_i as usize) ) % 26 ];
let new_char_3 =  get_rotor(r_r)[( new_char_2 as usize + (r_i as usize) ) % 26 ];
let new_char_4 =  get_reflector(reflector)[( new_char_3 as usize + (r_i as usize) ) % 26 ];
// reverse reflectors !
let new_char_5 =  get_rotor(r_r)[( new_char_4 as usize + (r_i as usize) ) % 26 ];
let new_char_6 =  get_rotor(r_r)[( new_char_5 as usize + (r_i as usize) ) % 26 ];
let new_char_7 =  get_rotor(r_r)[( new_char_6 as usize + (r_i as usize) ) % 26 ];

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
    (char, rotors)
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
    let new_l_rotor_i = if m_notches.contains(&m_i) || l_notches.contains(&r_i) {
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
        Rotor::I => vec![],
        Rotor::II => vec![],
        Rotor::III => vec![],
    }
}

fn get_reflector(r: Reflector) -> Vec<u8> {
  match r {
    Reflector::A => todo!(),
    Reflector::B => todo!(),
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

fn print_rotor(key: &str, str: &str) {
   print_any("Rotor", key, str)
}


fn print_reflector(key: &str, str: &str) {
  print_any("Reflector", key, str)
}

fn print_any(prefix:&str, key: &str, str: &str) {
  let mut offsets = Vec::new();
  for (i, c) in str.chars().enumerate() {
      offsets.push(char_utils::char_position_base(c).unwrap() + 25 - i);
  }
  println!("{}::{} =>\tvec!{:?},",prefix, key, offsets);
}


#[cfg(test)]
mod tests {
    use crate::enigma::{M3_settings, Reflector, Rotor};

    #[test]
    fn increment_1() {
        assert_eq!(
            M3_settings {
                reflector: Reflector::A,
                l_rotor: (Rotor::III, 0),
                m_rotor: (Rotor::II, 1),
                r_rotor: (Rotor::I, 17)
            },
            super::increment_rotors_m3(M3_settings {
                reflector: Reflector::A,
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
                reflector: Reflector::A,
                l_rotor: (Rotor::I, 1),
                m_rotor: (Rotor::II, 6),
                r_rotor: (Rotor::III, 24)
            },
            super::increment_rotors_m3(super::increment_rotors_m3(M3_settings {
                reflector: Reflector::A,
                l_rotor: (Rotor::I, 0),
                m_rotor: (Rotor::II, 4),
                r_rotor: (Rotor::III, 22)
            })),
        );
    }

    #[test]
    fn display_rotors() {
        super::print_rotor("I", "EKMFLGDQVZNTOWYHXUSPAIBRCJ");
        super::print_rotor("II", "AJDKSIRUXBLHWTMCQGZNPYFVOE");
        super::print_rotor("III", "BDFHJLCPRTXVZNYEIWGAKMUSQO");
        super::print_reflector("A", "EJMZALYXVBWFCRQUONTSPIKHGD");
        super::print_reflector("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT");
    }
}
