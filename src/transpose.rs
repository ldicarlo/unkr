use crate::models::NumberArgs;

pub fn decrypt(strs: Vec<String>, NumberArgs { number }: NumberArgs) -> Vec<String> {
    if number == 0 {
        return strs;
    }

    strs.iter()
        .flat_map(|str| {
            let mut results: Vec<Vec<char>> = Vec::new();
            let size: usize = str.len().try_into().unwrap();
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
                .map(|str| str.iter().collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec!["AD".to_string(), "BE".to_string(), "CF".to_string()],
            decrypt(vec!["ABCDEF".to_string()], NumberArgs { number: 3 })
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
            decrypt(
                vec!["ABCDEFGHIJKLMNO".to_string()],
                NumberArgs { number: 4 }
            )
        );
    }
}
