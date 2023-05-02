use crate::models::SwapArgs;

pub fn encrypt(strs: Vec<String>, order: Vec<usize>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in order {
        if let Some(str) = strs.get(i) {
            result.push(str.clone());
        }
    }
    for i in strs {
        if !result.contains(&i) {
            result.push(i);
        }
    }

    result
}

pub fn decrypt(strs: Vec<String>, SwapArgs { order }: SwapArgs) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut unordered_idx = order.len();
    for i in 0..strs.len() {
        if order.contains(&i) {
            result.push(strs[(order.iter().position(|&r| r == i).unwrap())].clone());
        } else {
            result.push(strs[unordered_idx].clone());
            unordered_idx = unordered_idx + 1;
        }
    }

    result
}

pub fn _decrypt(strs: Vec<String>, _seed: u64) -> Vec<String> {
    strs
}

#[cfg(test)]
mod tests {
    use crate::models::SwapArgs;

    #[test]
    fn it_works() {
        assert_eq!(
            super::encrypt(
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
                vec![2, 1, 0]
            ),
            vec!["ghi".to_string(), "def".to_string(), "abc".to_string(),]
        )
    }

    #[test]
    fn too_much_strings() {
        assert_eq!(
            super::encrypt(
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
                vec![2, 1]
            ),
            vec!["ghi".to_string(), "def".to_string(), "abc".to_string(),]
        )
    }

    #[test]
    fn back_and_forth() {
        assert_eq!(
            super::decrypt(
                super::encrypt(
                    vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
                    vec![2, 1]
                ),
                SwapArgs { order: vec![2, 1] }
            ),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        )
    }
}
