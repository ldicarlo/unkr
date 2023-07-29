use std::collections::{HashSet, VecDeque};

use crate::{
    models::{
        BruteForceCryptor, BruteForcePermuteArgs, BruteForceVigenereArgs, CLICryptor,
        CLIPermuteArgs, Cryptor, CryptorBase, DoneLine, HitLine, PartialLine,
        SerializablePartialLine, SerializablePartialLine2,
    },
    parser::{self, read_bruteforce_parameters},
};

pub fn cryptor_to_cli(cryptor: Cryptor) -> CLICryptor {
    match cryptor {
        Cryptor::Vigenere(args) => CLICryptor::Vigenere(args),
        Cryptor::Cut(args) => CLICryptor::Cut(args),
        Cryptor::Caesar(args) => CLICryptor::Caesar(args),
        Cryptor::Transpose(args) => CLICryptor::Transpose(args),
        Cryptor::AtBash => CLICryptor::AtBash,
        Cryptor::Reverse => CLICryptor::Reverse,
        Cryptor::Swap(args) => CLICryptor::Swap(args),
        Cryptor::Join => CLICryptor::Join,
        Cryptor::Colors(args) => CLICryptor::Colors(args),
        //  Cryptor::IndexCrypt(args) => CLICryptor::IndexCrypt(args),
        Cryptor::Permute(args) => CLICryptor::Permute(CLIPermuteArgs {
            permutations: args.permutations.into_iter().collect(),
        }),
        Cryptor::Enigma(args) => CLICryptor::Enigma(args),
        //  Cryptor::Reuse(args) => CLICryptor::Reuse(args),
    }
}

pub fn cryptor_base_to_string(cryptor: &CryptorBase) -> String {
    match cryptor {
        CryptorBase::Vigenere => todo!(),
        CryptorBase::Cut => todo!(),
        CryptorBase::Caesar => todo!(),
        CryptorBase::Transpose => todo!(),
        CryptorBase::AtBash => todo!(),
        CryptorBase::Reverse => String::from("Reverse"),
        CryptorBase::Swap => todo!(),
        CryptorBase::Join => todo!(),
        CryptorBase::IndexCrypt => todo!(),
        CryptorBase::Permute => String::from("Permute"),
        CryptorBase::Enigma => String::from("Enigma"),
    }
}

pub fn cryptor_base_from_cryptor(cryptor: &Cryptor) -> &CryptorBase {
    match cryptor {
        Cryptor::Vigenere(_) => &CryptorBase::Vigenere,
        Cryptor::Cut(_) => &CryptorBase::Cut,
        Cryptor::Caesar(_) => &CryptorBase::Caesar,
        Cryptor::Transpose(_) => &CryptorBase::Transpose,
        Cryptor::AtBash => &CryptorBase::AtBash,
        Cryptor::Reverse => &CryptorBase::Reverse,
        Cryptor::Swap(_) => &CryptorBase::Swap,
        Cryptor::Join => &CryptorBase::Join,
        Cryptor::Colors(_) => &CryptorBase::IndexCrypt,
        Cryptor::Permute(_) => &CryptorBase::Permute,
        Cryptor::Enigma(_) => &CryptorBase::Enigma,
    }
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
    PartialLine {
        cryptor: cryptor_to_cli(cryptor),
        tail,
    }
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
            BruteForceCryptor::Reuse(arg) => {
                (String::from("Reuse"), Some(cryptor_base_to_string(arg)))
            }
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
    let mut first_writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_writer(vec![]);
    first_writer
        .serialize(partial_line.clone().cryptor)
        .expect("FAIL");
    let first_str = String::from_utf8(first_writer.into_inner().expect("Cannot convert utf8"))
        .expect("Cannot convert utf8")
        .trim()
        .to_string();

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_writer(vec![]);

    writer
        .serialize(SerializablePartialLine2 {
            cryptor: first_str,
            tail: partial_line.clone().tail,
        })
        .expect(&format!("FAIL: {:?}", partial_line));
    String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
        .expect("Cannot convert utf8")
        .trim()
        .to_string()
}

// actually always returns one only
pub fn string_to_partial(str: String) -> Vec<PartialLine> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b';')
        .from_reader(str.as_bytes());
    rdr.records()
        .into_iter()
        .map(|result| {
            let mut record: SerializablePartialLine = result
                .expect("Failed to deserialize element.")
                .deserialize(None)
                .expect("Failed to deserialize element.");

            let head = parser::read_parameters(record.tail.pop_front().unwrap());
            PartialLine {
                cryptor: head,
                tail: record
                    .tail
                    .into_iter()
                    .map(read_bruteforce_parameters)
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::{
        enigma::{EnigmaArgs, Reflector, Rotor},
        models::{
            BruteForceCryptor, BruteForceVigenereArgs, CLICryptor, Cryptor, CryptorBase, DoneLine,
            PartialLine,
        },
        BruteForcePermuteArgs,
    };
    use serde::Deserialize;

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
    fn to_partial_to_string_works() {
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
                    }),
                    BruteForceCryptor::Permute(BruteForcePermuteArgs {
                        max_permutations: 3
                    }),
                    BruteForceCryptor::Reuse(CryptorBase::Permute)
                ]
                .into_iter()
                .collect()
            )),
            String::from("Enigma:B::I:1:II:6:III:24;Cut;Vigenere;3;4;Permute;3;Reuse;Permute")
        )
    }

    #[test]
    fn string_to_partial_works() {
        assert_eq!(
            super::string_to_partial(String::from("Reverse;Enigma;Enigma")),
            vec![PartialLine {
                cryptor: CLICryptor::Reverse,
                tail: vec![BruteForceCryptor::Enigma, BruteForceCryptor::Enigma]
                    .into_iter()
                    .collect()
            }]
        );
    }

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct Row {
        label: String,
        values: Vec<i32>,
    }
}
