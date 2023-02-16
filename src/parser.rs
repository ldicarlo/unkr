use super::models::{CryptorArgs, CryptorTypeWithArgs, NumberArgs, VigenereArgs};
use crate::models::{StringArgs, SwapArgs};

fn read(str: String, cryptor_type: CryptorTypeWithArgs) -> CryptorArgs {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(str.as_bytes());
    match cryptor_type {
        CryptorTypeWithArgs::Vigenere => CryptorArgs::Vigenere(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<VigenereArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Cut => CryptorArgs::Cut(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Caesar => CryptorArgs::Caesar(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Transpose => CryptorArgs::Transpose(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<NumberArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Swap => CryptorArgs::Swap(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<SwapArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::Colors => CryptorArgs::Colors(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<StringArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorTypeWithArgs::IndexCrypt => CryptorArgs::IndexCrypt(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<StringArgs>(None)
                .expect("cannot deserialize"),
        ),
    }
}

pub fn read_parameters(mut str: String) -> CryptorArgs {
    let type_name: String = str.drain(..str.find(':').unwrap_or(str.len())).collect();
    if str.len() > 0 {
        str.drain(0..1);
    }
    match type_name.as_str() {
        "vigenere" => read(str, CryptorTypeWithArgs::Vigenere),
        "cut" => read(str, CryptorTypeWithArgs::Cut),
        "transpose" => read(str, CryptorTypeWithArgs::Transpose),
        "reverse" => CryptorArgs::Reverse,
        "atbash" => CryptorArgs::AtBash,
        "swap" => read(str, CryptorTypeWithArgs::Swap),
        "join" => CryptorArgs::Join,
        "colors" => read(str, CryptorTypeWithArgs::Colors),
        "indexcrypt" => read(str, CryptorTypeWithArgs::IndexCrypt),
        _ => panic!("Cannot parse: {}", str),
    }
}

#[cfg(test)]
mod tests {

    use crate::models::SwapArgs;

    use super::{read, CryptorArgs, VigenereArgs};

    #[test]
    fn it_works() {
        assert_eq!(
            CryptorArgs::Vigenere(VigenereArgs {
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
            .serialize(CryptorArgs::Vigenere(VigenereArgs {
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
            .serialize(CryptorArgs::Swap(SwapArgs { order: vec![1, 2] }))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Swap:1:2\n".to_string())
    }

    #[test]
    fn read_whole_line() {
        assert_eq!(
            super::read_parameters(String::from("vigenere:KEY:ALPHABET")),
            CryptorArgs::Vigenere(VigenereArgs {
                key: String::from("KEY"),
                alphabet: String::from("ALPHABET")
            })
        )
    }
}
