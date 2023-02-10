/// ok fold but then we should reorder lines, so swap lines here or in another cryptor
pub fn decrypt(strs: Vec<String>, seed: u64) -> Vec<String> {
    if seed == 0 {
        return strs;
    }
    strs.iter()
        .flat_map(|str| {
            let mut result: Vec<char> = str.chars().clone().collect();
            let size: usize = str.len().try_into().unwrap();
            let block_size: usize = seed.try_into().unwrap();
            let mut lines_count: usize = size / block_size;
            // not the right way to do that ...
            if lines_count * block_size != size {
                lines_count = lines_count + 1;
            }
            let mut new_place = 0;
            for current_idx in 0..block_size {
                for line in 0..lines_count {
                    let old_place = line * block_size + current_idx;
                    if let Some(val) = str.chars().nth(old_place) {
                        result[new_place] = val;
                        // not the right way to do that ...
                        new_place = new_place + 1;
                    } else {
                        break;
                    }
                }
            }
            result.iter().collect::<String>().chars()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec!["ADBECF".to_string()],
            decrypt(vec!["ABCDEF".to_string()], 3)
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            vec!["AEIMBFJNCGKODHL".to_string()],
            decrypt(vec!["ABCDEFGHIJKLMNO".to_string()], 4)
        );
    }
}
