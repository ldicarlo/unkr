pub fn decrypt(str: String, seed: u64) -> String {
    let mut result: Vec<char> = str.chars().clone().collect();
    let size: usize = str.len().try_into().unwrap();
    let block_size: usize = seed.try_into().unwrap();
    let lines_count: usize = size / block_size;
    let mut new_place = 0;
    for current_idx in 0..block_size {
        for line in 0..lines_count {
            let old_place = line * block_size + current_idx;
            result[new_place] = str.chars().nth(old_place).unwrap();
            // not the right way to do that . . .
            new_place = new_place + 1;
        }
    }
    result.iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!("ADBECF".to_string(), decrypt("ABCDEF".to_string(), 3));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            "AEIMBFJNCGKODHL".to_string(),
            decrypt("ABCDEFGHIJKLMNO".to_string(), 4)
        );
    }
}
