use crate::{char_utils, models};

pub fn encrypt(
    strs: Vec<String>,
    models::StringArgs { letters }: models::StringArgs,
) -> Vec<String> {
    strs.into_iter()
        .map(|str| decrypt_string(str, letters.clone()))
        .collect()
}

pub fn decrypt(
    strs: Vec<String>,
    models::StringArgs { letters }: models::StringArgs,
) -> Vec<String> {
    strs.into_iter()
        .map(|str| decrypt_string(str, letters.clone()))
        .collect()
}

fn decrypt_string(string: String, base: String) -> String {
    let mut new_base = base.clone();
    let mut result: Vec<char> = vec![];
    for i in 0..string.len() {
        let idx = char_utils::char_position_base(string.chars().nth(i).unwrap()).unwrap();
        result.push(new_base.remove(idx % new_base.len()));
    }
    result.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            // H TJOSHDELSDL -> E
            // E TJOSDELSDL -> F
            // L TJOSDLSDL -> F
            // L TJOSDSDL -> H
            // O TJOSDSD -> C
            super::decrypt_string("EFFHC".to_string(), "TJOSHDELSDL".to_string()),
            "HELLO".to_string()
        );
    }
}
