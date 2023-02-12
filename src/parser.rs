#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
#[serde(tag = "name", deny_unknown_fields)]
enum CryptorsArgs {
    Vigenere(VigenereArgs),
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
struct VigenereArgs {
    pub key: String,
    pub alphabet: String,
}

fn read(str: String) -> Vec<CryptorsArgs> {
    let mut result = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(str.as_bytes());
    for record in rdr.records() {
        if let Ok(line) = record {
            let li = line.deserialize::<CryptorsArgs>(None);
            if let Ok(user) = li {
                result.push(user);
            } else {
                println!("cannot deserialize, {:?}", li);
            }
        } else {
            println!("cannot deserialize, {:?}", record);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{read, CryptorsArgs, VigenereArgs};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![CryptorsArgs::Vigenere(VigenereArgs {
                key: "K".to_string(),
                alphabet: "ALP".to_string()
            })],
            read("Vigenere:K:ALP\n".to_string())
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
}
