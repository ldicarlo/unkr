use crate::{
    base,
    char_utils::{self, get_alphabet_prefixed},
};

pub fn fuzz_from(str: String, len_max: usize, base: usize, rules: Vec<String>) {
    let mut effective_rules: Vec<Box<dyn Fn(Vec<u8>) -> bool>> = vec![];
    if rules.contains(&String::from("UniqueLetters")) {
        effective_rules.push(Box::new(unique_letters));
    }
    if rules.contains(&String::from("EvenCount")) {
        effective_rules.push(Box::new(pair_length));
    }
    // if rules.contains(&String::from("SortedLettersByPair")) {
    //     effective_rules.push(Box::new(sorted_letters_by_pair));
    // }

    let mut last = str;
    while let Some(next) = fuzz_next_string_ruled(last.clone(), len_max, base, &effective_rules) {
        println!("{}", next);
        last = next;
    }
}

pub fn fuzz_next_string_ruled(
    str: String,
    len_max: usize,
    base: usize,
    rules: &Vec<Box<dyn Fn(Vec<u8>) -> bool>>,
) -> Option<String> {
    fuzz_next_r(
        str.chars()
            .into_iter()
            .flat_map(|c| char_utils::char_position(c, get_alphabet_prefixed()))
            .map(|c| c as u8)
            .collect(),
        len_max,
        base,
        rules,
    )
    .map(|vec| {
        vec.into_iter()
            .map(|c| get_alphabet_prefixed()[c as usize])
            .collect::<String>()
    })
}

pub fn fuzz_next_r(
    str: Vec<u8>,
    len_max: usize,
    base: usize,
    rules: &Vec<Box<dyn Fn(Vec<u8>) -> bool>>,
) -> Option<Vec<u8>> {
    let mut last = str;
    while let Some(result) = fuzz_next(last, len_max, base) {
        last = result.clone();
        if rules.iter().all(|f| f(last.clone())) {
            return Some(last);
        }
    }
    None
}

pub fn fuzz_next(str: Vec<u8>, len_max: usize, base: usize) -> Option<Vec<u8>> {
    let vector: Vec<u8> = str.clone().into_iter().map(|c| c as u8).collect();
    if str.len() == len_max && vector.clone().into_iter().all(|c| c as usize == base - 1) {
        return None;
    }
    Some(base::increment(vector, base))
}

pub fn fuzz_next_string_bases(str: String, bases: Vec<usize>) -> Option<String> {
    fuzz_next_bases(
        str.chars()
            .into_iter()
            .flat_map(|c| char_utils::char_position(c, get_alphabet_prefixed()))
            .map(|c| c as u8)
            .collect(),
        bases,
    )
    .map(|vec| {
        vec.into_iter()
            .map(|c| get_alphabet_prefixed()[c as usize])
            .collect::<String>()
    })
}

pub fn fuzz_next_bases(str: Vec<u8>, bases: Vec<usize>) -> Option<Vec<u8>> {
    let vector: Vec<u8> = str.clone().into_iter().map(|c| c as u8).collect();
    if vector
        .clone()
        .into_iter()
        .enumerate()
        .all(|(i, c)| c as usize == bases[i] - 1)
    {
        return None;
    }
    Some(base::increment_with_bases(vector, bases))
}

pub fn unique_letters(str: Vec<u8>) -> bool {
    let mut vec = vec![];
    for num in str.into_iter() {
        if vec.contains(&num) {
            return false;
        }
        vec.push(num);
    }
    return true;
}

pub fn pair_length(str: Vec<u8>) -> bool {
    str.len() % 2 == 0
}

pub fn sorted_letters_by_pair(str: Vec<u8>, containing: Vec<u8>) -> bool {
    let base: Vec<(u8, u8)> = char_utils::vec_to_pairs(str);

    if !base
        .iter()
        .all(|(a, b)| containing.contains(a) || containing.contains(b))
    {
        return false;
    }

    let mut ordered = base
        .clone()
        .into_iter()
        .map(|(a, b)| if a > b { (b, a) } else { (a, b) })
        .collect::<Vec<(u8, u8)>>();
    ordered.sort_by(|(a, _), (b, _)| a.cmp(b));

    base == ordered //&& str.iter()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(
            super::fuzz_next_string_ruled("KRYPTOR".to_string(), 7, 27, &vec![]),
            Some("KRYPTOS".to_string())
        );
        assert_eq!(
            super::fuzz_next_string_ruled("ZZZ".to_string(), 3, 27, &vec![]),
            None
        );
        assert_eq!(
            super::fuzz_next_string_ruled("ZZ".to_string(), 3, 27, &vec![]),
            Some("AAA".to_string())
        );
    }

    #[test]
    fn ordered_works() {
        assert_eq!(
            super::sorted_letters_by_pair(vec![1, 2, 4, 5], vec![1, 4]),
            true
        );
        assert_eq!(
            super::sorted_letters_by_pair(vec![1, 2, 5, 4], vec![1, 5]),
            false
        );
        assert_eq!(
            super::sorted_letters_by_pair(vec![2, 1, 5, 4], vec![2, 5]),
            false
        );
        assert_eq!(
            super::sorted_letters_by_pair(vec![2, 1, 4, 5, 6, 3], vec![2, 4, 6]),
            false
        );
        assert_eq!(
            super::sorted_letters_by_pair(vec![1, 2, 3, 4], vec![5, 6]),
            false
        );
    }

    #[test]
    fn stop_at_end() {
        assert_eq!(
            super::fuzz_next_bases(vec![1, 3, 26, 3, 26, 3, 26], vec![2, 4, 27, 4, 27, 4, 27]),
            None
        );
    }
}
