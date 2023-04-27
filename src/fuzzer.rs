use crate::{
    base,
    char_utils::{self, get_alphabet},
};

pub fn fuzz_from(str: String, len_max: usize) {
    let mut last = str;
    println!("{}", last);
    while let Some(next) = fuzz_next(last.clone(), len_max) {
        println!("{}", next);
        last = next;
    }
}

pub fn fuzz_next(str: String, len_max: usize) -> Option<String> {
    let vector: Vec<u8> = str
        .chars()
        .flat_map(char_utils::char_position_base)
        .map(|c| c as u8)
        .collect();
    if str.len() == len_max && vector.clone().into_iter().all(|c| c as usize == 25) {
        return None;
    }
    Some(
        base::increment(vector, 26)
            .into_iter()
            .map(|c| get_alphabet()[c as usize])
            .collect(),
    )
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
            Some("AZZ".to_string())
        );
    }
}
