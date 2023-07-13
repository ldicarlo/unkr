use std::collections::{HashSet, VecDeque};

use crate::models::{
    BruteForceCryptor, BruteForcePermuteArgs, BruteForceVigenereArgs, CLICryptor, Cryptor,
    DoneLine, HitLine, PartialLine,
};

pub fn cryptor_to_cli(cryptor: Cryptor)
// -> CLICryptor
{
}

pub fn hit_to_string(hit_line: HitLine) -> String {
    format!("{};{:?}", hit_line.result, hit_line.args)
}

pub fn to_done(combination: VecDeque<BruteForceCryptor>) -> DoneLine {
    let (left, right) = combinations_string(combination);
    DoneLine {
        combinations: left,
        args: right,
    }
}

pub fn to_partial(cryptor: Cryptor, tail: VecDeque<BruteForceCryptor>) -> PartialLine {
    PartialLine { cryptor, tail }
}

pub fn combinations_string(
    brute_force_cryptors: VecDeque<BruteForceCryptor>,
) -> (String, Option<String>) {
    let strings: Vec<(String, Option<String>)> = brute_force_cryptors
        .iter()
        .map(|c| match c {
            BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                alphabet_depth,
                key_depth,
            }) => (
                String::from("Vigenere"),
                Some(format!("Vigenere:{}:{}", alphabet_depth, key_depth)),
            ),
            BruteForceCryptor::Cut => (String::from("Cut"), None),
            BruteForceCryptor::Caesar => (String::from("Caesar"), None),
            BruteForceCryptor::Transpose => (String::from("Transpose"), None),
            BruteForceCryptor::AtBash => (String::from("AtBash"), None),
            BruteForceCryptor::Reverse => (String::from("Reverse"), None),
            BruteForceCryptor::Swap => (String::from("Swap"), None),
            BruteForceCryptor::Join => (String::from("Join"), None),
            //    BruteForceCryptor::IndexCrypt => (String::from("IndexCrypt"), None),
            BruteForceCryptor::Permute(BruteForcePermuteArgs { max_permutations }) => (
                String::from("Permute"),
                Some(format!("Permute:{}", max_permutations)),
            ),
            BruteForceCryptor::Enigma => (String::from("Enigma"), None),
        })
        .collect();
    //strings.sort_by_key(|(a, _)| a.clone());
    let left = strings
        .clone()
        .into_iter()
        .map(|(a, _)| a)
        .collect::<Vec<String>>()
        .join(" ");

    let rights = strings
        .into_iter()
        .flat_map(|(_, b)| b)
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    let right = if rights.is_empty() {
        None
    } else {
        Some(rights.join(" "))
    };

    (left, right)
}

pub fn partial_to_string(partial_line: PartialLine) -> String {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_writer(vec![]);

    writer.serialize(partial_line.clone()).expect("FAIL");
    String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
        .expect("Cannot convert utf8")
        .trim()
        .to_string()
}

#[cfg(test)]
pub mod tests {
    use crate::{
        enigma::{EnigmaArgs, Reflector, Rotor},
        models::{BruteForceCryptor, BruteForceVigenereArgs, Cryptor, DoneLine},
    };

    #[test]
    fn to_done_works() {
        assert_eq!(
            super::to_done(
                vec![
                    BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                        alphabet_depth: 4,
                        key_depth: 7
                    },),
                    BruteForceCryptor::Transpose,
                    BruteForceCryptor::Caesar,
                ]
                .into(),
            ),
            DoneLine {
                combinations: String::from("Vigenere Transpose Caesar"),
                args: Some(String::from("Vigenere:4:7"))
            }
        )
    }

    #[test]
    fn to_done_no_args_works() {
        assert_eq!(
            super::to_done(vec![BruteForceCryptor::Transpose, BruteForceCryptor::Caesar,].into(),),
            DoneLine {
                combinations: String::from("Transpose Caesar"),
                args: None
            }
        )
    }

    #[test]
    fn to_partial_to_string() {
        assert_eq!(
            super::partial_to_string(super::to_partial(
                Cryptor::Enigma(EnigmaArgs {
                    reflector: Reflector::B,
                    l0_rotor: None,
                    l_rotor: (Rotor::I, 1),
                    m_rotor: (Rotor::II, 6),
                    r_rotor: (Rotor::III, 24)
                },),
                vec![
                    BruteForceCryptor::Cut,
                    BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
                        alphabet_depth: 3,
                        key_depth: 4
                    })
                ]
                .into_iter()
                .collect()
            )),
            String::from("Enigma;B;;I;1;II;6;III;24;Cut;Vigenere;3;4")
        )
    }
}
