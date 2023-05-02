use std::collections::HashSet;

use crate::{
    base,
    char_utils::{self, get_alphabet_prefixed},
};

pub fn fuzz_from(str: String, len_max: usize) {
    let mut last = str;
    while let Some(next) = fuzz_next_r(
        last.clone(),
        len_max,
        vec![
            Box::new(pair_length),
            Box::new(unique_letters),
            Box::new(sorted_letters_by_pair),
        ],
    ) {
        println!("{}", next);
        last = next;
    }
}

pub fn fuzz_next_r(
    str: String,
    len_max: usize,
    rules: Vec<Box<dyn Fn(String) -> bool>>,
) -> Option<String> {
    let mut last = str;
    while let Some(result) = fuzz_next(last, len_max) {
        last = result.clone();
        if rules.iter().all(|f| f(last.clone())) {
            return Some(last);
        }
    }
    None
}

pub fn fuzz_next(str: String, len_max: usize) -> Option<String> {
    let vector: Vec<u8> = str
        .chars()
        .flat_map(|c| char_utils::char_position(c, get_alphabet_prefixed()))
        .map(|c| c as u8)
        .collect();
    if str.len() == len_max && vector.clone().into_iter().all(|c| c as usize == 26) {
        return None;
    }
    Some(
        base::increment(vector, 27)
            .into_iter()
            .map(|c| get_alphabet_prefixed()[c as usize])
            .collect(),
    )
}

pub fn unique_letters(str: String) -> bool {
    str.len() == str.chars().into_iter().collect::<HashSet<char>>().len()
}

pub fn pair_length(str: String) -> bool {
    str.len() % 2 == 0
}

pub fn sorted_letters_by_pair(str: String) -> bool {
    let base: Vec<(char, char)> = char_utils::string_to_vec(str);

    let mut ordered = base
        .clone()
        .into_iter()
        .map(|(a, b)| if a > b { (b, a) } else { (a, b) })
        .collect::<Vec<(char, char)>>();
    ordered.sort_by(|(a, _), (b, _)| a.cmp(b));

    base == ordered
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            super::fuzz_next("KRYPTOR".to_string(), 7),
            Some("KRYPTOS".to_string())
        );
        assert_eq!(super::fuzz_next("ZZZ".to_string(), 3), None);
        assert_eq!(
            super::fuzz_next("ZZ".to_string(), 3),
            Some("AAA".to_string())
        );
    }

    #[test]
    fn ordered_works() {
        assert_eq!(super::sorted_letters_by_pair("ADFG".to_string()), true);
        assert_eq!(super::sorted_letters_by_pair("ADGF".to_string()), false);
        assert_eq!(super::sorted_letters_by_pair("DAGF".to_string()), false);
        assert_eq!(super::sorted_letters_by_pair("DAGFHE".to_string()), false);
    }
}
