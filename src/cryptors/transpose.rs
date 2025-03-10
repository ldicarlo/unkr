use crate::models::NumberArgs;

pub fn init() -> NumberArgs {
    NumberArgs { number: 1 }
}

pub fn next(strs: Vec<String>, NumberArgs { number }: NumberArgs) -> Option<NumberArgs> {
    if number < strs[0].len() - 1 {
        Some(NumberArgs { number: number + 1 })
    } else {
        None
    }
}

pub fn encrypt(strs: Vec<String>, NumberArgs { number }: NumberArgs) -> Vec<String> {
    let str = strs.join("");
    let mut padded_str = str.clone();
    for _ in 0..number - (str.len() % number) {
        padded_str.push(' ');
    }

    internal_encrypt(padded_str, number)
}

pub fn decrypt(strs: Vec<String>, NumberArgs { number }: NumberArgs) -> Vec<String> {
    let str = strs.join("");
    let size = str.len();
    let decrypt_number = if size % number == 0 {
        size / number
    } else {
        size / number + 1
    };
    let str = strs.join("");
    let mut padded_str = str.clone();
    for i in 0..number - (str.len() % number) {
        padded_str.insert(padded_str.len() - (decrypt_number * i), ' ');
    }

    internal_encrypt(padded_str, decrypt_number)
}

fn internal_encrypt(str: String, number: usize) -> Vec<String> {
    let size: usize = str.len();
    let mut results: Vec<Vec<char>> = Vec::new();
    let block_size: usize = number.try_into().unwrap();
    let mut lines_count: usize = size / block_size;
    // not the right way to do that ...
    if lines_count * block_size != size {
        lines_count = lines_count + 1;
    }

    for current_idx in 0..block_size {
        let mut current_line = Vec::new();
        for line in 0..lines_count {
            let old_place = line * block_size + current_idx;
            if let Some(val) = str.chars().nth(old_place) {
                current_line.push(val);
            } else {
                break;
            }
        }
        if current_line.len() > 0 {
            results.push(current_line);
        }
    }
    results
        .iter()
        .map(|str| str.iter().collect::<String>().trim_end().to_string())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec!["AD".to_string(), "BE".to_string(), "CF".to_string()],
            encrypt(vec!["ABCDEF".to_string()], NumberArgs { number: 3 })
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            vec![
                "AEIM".to_string(),
                "BFJN".to_string(),
                "CGKO".to_string(),
                "DHL".to_string(),
            ],
            encrypt(
                vec!["ABCDEFGHIJKLMNO".to_string()],
                NumberArgs { number: 4 }
            )
        );
    }
    #[test]
    fn encrypt_2() {
        assert_eq!(
            vec![
                "AEIM".to_string(),
                "BFJN".to_string(),
                "CGKO".to_string(),
                "DHLP".to_string(),
            ],
            encrypt(
                vec!["ABCDEFGHIJKLMNOP".to_string()],
                NumberArgs { number: 4 }
            )
        );
    }
}
