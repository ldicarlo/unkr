pub fn find_and_print_candidates(strs: Vec<String>, clues: Vec<String>) -> Vec<String> {
    let candidates = find_candidates(strs.clone(), clues.clone());
    if candidates.len() > 0 {
        println!("{:?} {:?}", candidates, strs);
    }
    candidates
}
pub fn find_candidates(strs: Vec<String>, clues: Vec<String>) -> Vec<String> {
  check_string_for_candidates(&strs.join(""), &clues)

}

fn check_string_for_candidates(string: &String, clues: &Vec<String>) -> Vec<String> {
    let step1 = clue_is_in_string(string, clues);
    if step1.len() > 0 {
        return step1;
    }
    step1
}

fn clue_is_in_string(string: &String, clues: &Vec<String>) -> Vec<String> {
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
            clue_is_in_string(&String::from("STRING"), &vec![String::from("IN")]),
            vec!["IN was found in STRING"]
        )
    }
}
