use super::models::{CryptorType,CryptorsArgs,VigenereArgs,SimpleArgs};

fn read(str: String, cryptor_type: CryptorType) -> CryptorsArgs {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(str.as_bytes());
    match cryptor_type {
        CryptorType::Vigenere => CryptorsArgs::Vigenere(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<VigenereArgs>(None)
                .expect("cannot deserialize"),
        ),
        CryptorType::Cut => CryptorsArgs::Cut(
            rdr.records()
                .find(|_| true)
                .unwrap()
                .expect("cannot find record")
                .deserialize::<SimpleArgs>(None)
                .expect("cannot deserialize"),
        ),
    }
}

pub fn read_parameters(mut str: String) -> (CryptorType, CryptorsArgs) {
    let type_name: String = str.drain(..str.find(':').unwrap()).collect();
    str.drain(0..1);
    match type_name.as_str() {
        "vigenere" => (CryptorType::Vigenere, read(str, CryptorType::Vigenere)),
        "cut" => (CryptorType::Cut, read(str, CryptorType::Cut)),
        _ => panic!("Cannot parse: {}", str),
    }
}

pub fn read_line(str: String) -> Vec<(CryptorType, CryptorsArgs)> {
    let split: Vec<&str> = str.split(' ').collect();
    split
        .iter()
        .map(|c| c.clone().to_string())
        .map(read_parameters)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::models::CryptorType;

    use super::{read, CryptorsArgs, VigenereArgs};

    #[test]
    fn it_works() {
        assert_eq!(
            CryptorsArgs::Vigenere(VigenereArgs {
                key: "K".to_string(),
                alphabet: "ALP".to_string()
            }),
            read("K:ALP".to_string(), super::CryptorType::Vigenere)
        )
    }

    #[test]
    fn it_works_2() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_writer(vec![]);

        writer
            .serialize(CryptorsArgs::Vigenere(VigenereArgs {
                key: "K".to_string(),
                alphabet: "ALP".to_string(),
            }))
            .expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");

        assert_eq!(result, "Vigenere:K:ALP\n".to_string())
    }
    #[test]
    fn read_whole_line() {
        assert_eq!(
            super::read_line(String::from("vigenere:KEY:ALPHABET cut:2")),
            vec![
                (
                    CryptorType::Vigenere,
                    CryptorsArgs::Vigenere(VigenereArgs {
                        key: String::from("KEY"),
                        alphabet: String::from("ALPHABET")
                    })
                ),
                (
                    CryptorType::Cut,
                    CryptorsArgs::Cut(super::SimpleArgs { number: 2 })
                )
            ]
        )
    }
}
