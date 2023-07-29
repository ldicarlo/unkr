use super::models::{CryptorTypeWithArgs, NumberArgs, VigenereArgs};
use crate::{
    enigma::EnigmaArgs,
    models::{
        BruteForceCryptor, BruteForcePermuteArgs, BruteForceVigenereArgs, CLICryptor,
        CLIPermuteArgs, CryptorBase, CryptorTypeWithBruteForceArgs, StringArgs, SwapArgs,
    },
};

fn read(str: String, cryptor_type: CryptorTypeWithArgs) -> CLICryptor {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(str.as_bytes());
    match cryptor_type {
        CryptorTypeWithArgs::Vigenere => CLICryptor::Vigenere(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<VigenereArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Cut => CLICryptor::Cut(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Caesar => CLICryptor::Caesar(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Transpose => CLICryptor::Transpose(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Swap => CLICryptor::Swap(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<SwapArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Colors => CLICryptor::Colors(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<StringArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::IndexCrypt => CLICryptor::IndexCrypt(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<StringArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Permute => CLICryptor::Permute(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<CLIPermuteArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Enigma => CLICryptor::Enigma(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<EnigmaArgs>(None)
                .expect("cannot deserialize"),
        ),
    }
}

pub fn read_parameters(mut str: String) -> CLICryptor {
    let type_name: String = str.drain(..str.find(':').unwrap_or(str.len())).collect();
    if str.len() > 0 {
        str.drain(0..1);
    }
    match type_name.to_lowercase().as_str() {
        "vigenere" => read(str, CryptorTypeWithArgs::Vigenere),
        "cut" => read(str, CryptorTypeWithArgs::Cut),
        "transpose" => read(str, CryptorTypeWithArgs::Transpose),
        "reverse" => CLICryptor::Reverse,
        "atbash" => CLICryptor::AtBash,
        "swap" => read(str, CryptorTypeWithArgs::Swap),
        "join" => CLICryptor::Join,
        "colors" => read(str, CryptorTypeWithArgs::Colors),
        "indexcrypt" => read(str, CryptorTypeWithArgs::IndexCrypt),
        "permute" => read(str, CryptorTypeWithArgs::Permute),
        "enigma" => read(str, CryptorTypeWithArgs::Enigma),
        _ => panic!("Cannot parse: {}", str),
    }
}

pub fn read_bruteforce_parameters(mut str: String) -> BruteForceCryptor {
    let type_name: String = str.drain(..str.find(':').unwrap_or(str.len())).collect();
    if str.len() > 0 {
        str.drain(0..1);
    }
    match type_name.to_lowercase().as_str() {
        "vigenere" => read_bruteforce(str, CryptorTypeWithBruteForceArgs::Vigenere),
        "cut" => BruteForceCryptor::Cut,
        "transpose" => BruteForceCryptor::Transpose,
        "reverse" => BruteForceCryptor::Reverse,
        "atbash" => BruteForceCryptor::AtBash,
        "swap" => BruteForceCryptor::Swap,
        "join" => BruteForceCryptor::Join,
        //   "indexcrypt" => BruteForceCryptor::IndexCrypt,
        "permute" => read_bruteforce(str, CryptorTypeWithBruteForceArgs::Permute),
        "enigma" => BruteForceCryptor::Enigma,
        "reuse" => read_bruteforce(str, CryptorTypeWithBruteForceArgs::Reuse),
        _ => panic!("Cannot parse: {}", type_name),
    }
}

fn read_bruteforce(str: String, cryptor_type: CryptorTypeWithBruteForceArgs) -> BruteForceCryptor {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(str.as_bytes());
    match cryptor_type {
        CryptorTypeWithBruteForceArgs::Vigenere => BruteForceCryptor::Vigenere(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<BruteForceVigenereArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithBruteForceArgs::Permute => BruteForceCryptor::Permute(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<BruteForcePermuteArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithBruteForceArgs::Reuse => BruteForceCryptor::Reuse(read_cryptor_base(str)),
    }
}

fn read_cryptor_base(str: String) -> CryptorBase {
    match str.to_lowercase().as_str() {
        "vigenere" => CryptorBase::Vigenere,
        "cut" => CryptorBase::Cut,
        "transpose" => CryptorBase::Transpose,
        "reverse" => CryptorBase::Reverse,
        "atbash" => CryptorBase::AtBash,
        "swap" => CryptorBase::Swap,
        "join" => CryptorBase::Join,
        "indexcrypt" => CryptorBase::IndexCrypt,
        "permute" => CryptorBase::Permute,
        "enigma" => CryptorBase::Enigma,
        _ => panic!("Cannot parse: {}", str),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        enigma::{EnigmaArgs, Reflector, Rotor},
        models::{BruteForceCryptor, CLICryptor, CLIPermuteArgs, SwapArgs},
    };

    use super::{read, VigenereArgs};

    #[test]
    fn all_cryptors_readable() {}

    #[test]
    fn it_works() {
        assert_eq!(
            CLICryptor::Vigenere(VigenereArgs {
                key: "K".to_string(),
                alphabet: "ALP".to_string()
            }),
            read("K:ALP".to_string(), super::CryptorTypeWithArgs::Vigenere)
        )
    }

    #[test]
    fn it_works_2() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(CLICryptor::Vigenere(VigenereArgs {
                key: "K".to_string(),
                alphabet: "ALP".to_string(),
            }))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Vigenere:K:ALP\n".to_string())
    }

    #[test]
    fn swap_serialize() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(CLICryptor::Swap(SwapArgs { order: vec![1, 2] }))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Swap:1:2\n".to_string())
    }

    #[test]
    fn read_whole_line() {
        assert_eq!(
            super::read_parameters(String::from("vigenere:KEY:ALPHABET")),
            CLICryptor::Vigenere(VigenereArgs {
                key: String::from("KEY"),
                alphabet: String::from("ALPHABET")
            })
        )
    }

    #[test]
    fn permute_serialize() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(CLICryptor::Permute(CLIPermuteArgs {
                permutations: vec![('A', 'B'), ('C', 'D')].into_iter().collect(),
            }))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Permute:A:B:C:D\n".to_string())
    }

    #[test]
    fn permute_bruteforce_serialize() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(BruteForceCryptor::Permute(
                crate::models::BruteForcePermuteArgs {
                    max_permutations: 6,
                },
            ))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Permute:6\n".to_string())
    }

    #[test]
    fn vigenere_bruteforce_serialize() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(BruteForceCryptor::Vigenere(
                crate::models::BruteForceVigenereArgs {
                    alphabet_depth: 4,
                    key_depth: 5,
                },
            ))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Vigenere:4:5\n".to_string())
    }

    #[test]
    fn enigma_serialize() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(EnigmaArgs {
                reflector: Reflector::B,
                l0_rotor: None,
                l_rotor: (Rotor::I, 0),
                m_rotor: (Rotor::II, 0),
                r_rotor: (Rotor::III, 0),
            })
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "B::I:0:II:0:III:0\n".to_string())
    }

    #[test]
    fn enigma_serialize_4_rotors() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(EnigmaArgs {
                reflector: Reflector::B,
                l0_rotor: Some((Rotor::I, 0)),
                l_rotor: (Rotor::I, 0),
                m_rotor: (Rotor::II, 0),
                r_rotor: (Rotor::III, 0),
            })
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "B:I:0:I:0:II:0:III:0\n".to_string())
    }
}
