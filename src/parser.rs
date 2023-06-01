use super::models::{Cryptor, CryptorTypeWithArgs, NumberArgs, VigenereArgs};
use crate::{
    enigma::EnigmaArgs,
    models::{
        BruteForceCryptor, BruteForcePermuteArgs, BruteForceVigenereArgs,
        CryptorTypeWithBruteForceArgs, PermuteArgs, StringArgs, SwapArgs,
    },
};

fn read(str: String, cryptor_type: CryptorTypeWithArgs) -> Cryptor {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(str.as_bytes());
    match cryptor_type {
        CryptorTypeWithArgs::Vigenere => Cryptor::Vigenere(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<VigenereArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Cut => Cryptor::Cut(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Caesar => Cryptor::Caesar(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Transpose => Cryptor::Transpose(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Swap => Cryptor::Swap(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<SwapArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Colors => Cryptor::Colors(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<StringArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::IndexCrypt => Cryptor::IndexCrypt(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<StringArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Permute => Cryptor::Permute(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<PermuteArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Enigma => Cryptor::Enigma(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<EnigmaArgs>(None)
                .expect("cannot deserialize"),
        ),
    }
}

pub fn read_parameters(mut str: String) -> Cryptor {
    let type_name: String = str.drain(..str.find(':').unwrap_or(str.len())).collect();
    if str.len() > 0 {
        str.drain(0..1);
    }
    match type_name.to_lowercase().as_str() {
        "vigenere" => read(str, CryptorTypeWithArgs::Vigenere),
        "cut" => read(str, CryptorTypeWithArgs::Cut),
        "transpose" => read(str, CryptorTypeWithArgs::Transpose),
        "reverse" => Cryptor::Reverse,
        "atbash" => Cryptor::AtBash,
        "swap" => read(str, CryptorTypeWithArgs::Swap),
        "join" => Cryptor::Join,
        "colors" => read(str, CryptorTypeWithArgs::Colors),
        "indexcrypt" => read(str, CryptorTypeWithArgs::IndexCrypt),
        "permute" => read(str, CryptorTypeWithArgs::Permute),
        "enigma" => read(str, CryptorTypeWithArgs::Enigma),
        _ => panic!("Cannot parse: {}", str),
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
    }
}

pub fn read_bruteforce_parameters(mut str: String) -> BruteForceCryptor {
    let type_name: String = str.drain(..str.find(':').unwrap_or(str.len())).collect();
    if str.len() > 0 {
        str.drain(0..1);
    }
    match type_name.as_str() {
        "vigenere" => read_bruteforce(str, CryptorTypeWithBruteForceArgs::Vigenere),
        "cut" => BruteForceCryptor::Cut,
        "transpose" => BruteForceCryptor::Transpose,
        "reverse" => BruteForceCryptor::Reverse,
        "atbash" => BruteForceCryptor::AtBash,
        "swap" => BruteForceCryptor::Swap,
        "join" => BruteForceCryptor::Join,
        "indexcrypt" => BruteForceCryptor::IndexCrypt,
        "permute" => read_bruteforce(str, CryptorTypeWithBruteForceArgs::Permute),
        _ => panic!("Cannot parse: {}", str),
    }
}

#[cfg(test)]
mod tests {

    use crate::{models::{BruteForceCryptor, PermuteArgs, SwapArgs}, enigma::{EnigmaArgs, M3_settings, Reflector, Rotor}};

    use super::{read, Cryptor, VigenereArgs};

    #[test]
    fn all_cryptors_readable() {}

    #[test]
    fn it_works() {
        assert_eq!(
            Cryptor::Vigenere(VigenereArgs {
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
            .serialize(Cryptor::Vigenere(VigenereArgs {
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
            .serialize(Cryptor::Swap(SwapArgs { order: vec![1, 2] }))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Swap:1:2\n".to_string())
    }

    #[test]
    fn read_whole_line() {
        assert_eq!(
            super::read_parameters(String::from("vigenere:KEY:ALPHABET")),
            Cryptor::Vigenere(VigenereArgs {
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
            .serialize(Cryptor::Permute(PermuteArgs {
                permutations: vec![('A', 'B'), ('C', 'D')],
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
            .serialize(Cryptor::Enigma( EnigmaArgs::M3(M3_settings {
              reflector: Reflector::B,
              l_rotor: (Rotor::I, 0),
              m_rotor: (Rotor::II, 0),
              r_rotor: (Rotor::III, 0)
          })))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Vigenere:4:5\n".to_string())
    }
}
