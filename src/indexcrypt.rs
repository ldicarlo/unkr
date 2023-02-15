use crate::char_utils;

pub fn decrypt(strs: Vec<String>, base: String) -> Vec<String> {
    strs.into_iter()
        .map(|str| decrypt_string(str, base.clone()))
        .collect()
}

fn decrypt_string(string: String, base: String) -> String {
    let mut new_base = base.clone();
    let mut result: Vec<char> = vec![];
    for i in 0..string.len() {
        //println!("{}\t{} {}", i, result.clone().into_iter().collect::<String>(), new_base);
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
