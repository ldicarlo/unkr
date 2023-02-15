pub fn is_candidate(strs: Vec<String>, clues: Vec<String>) -> Vec<String> {
    vec![]
}

fn clue_is_in_string(string: String, clues: Vec<String>) -> Vec<String> {
    clues
        .iter()
        .filter(|clue| string.contains(*clue))
        .map(|clue| format!("{} was found in {}", clue, string))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            clue_is_in_string(String::from("STRING"), vec![String::from("IN")]),
            vec!["IN was found in STRING"]
        )
    }
}
