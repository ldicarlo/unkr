/// ok fold but then we should reorder lines, so swap lines here or in another cryptor
///
///
/// ABCDEF / blocksize 3
/// # Step 1
/// A|B|C
/// D|E|F
/// 
/// # Step 2
/// AD, BE, CF
pub fn decrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    if seed == 0 {
        return strs;
    }

    strs.iter()
        .flat_map(|str| {
            let mut results: Vec<Vec<char>> = Vec::new();
            let size: usize = str.len().try_into().unwrap();
            let block_size: usize = seed.try_into().unwrap();
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
                results.push(current_line);
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
            vec!["AD".to_string(),"BE".to_string(), "CF".to_string()],
            decrypt(vec!["ABCDEF".to_string()], 3)
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
            decrypt(vec!["ABCDEFGHIJKLMNO".to_string()], 4)
        );
    }
}
