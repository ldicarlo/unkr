use std::collections::HashMap;

/// Get all possible combinations
/// A,B,C ->
/// - A
/// - B
/// - C
/// - A,B
/// - A,C
/// - B,A
/// - B,C
/// - C,A
/// - C,B
/// - A {x0,x1,x2},B,C
/// - A,C,B
/// - B,A,C
/// - B,C,A
/// - C,B,A
/// - C,A,B

type Combination = Vec<Vec<Step>>;

struct Step {
    decryptor: String,
    executions: u8,
}

pub fn get_combinations(vector: Vec<&str>, range: Vec<u8>) {}

pub fn combinate_strings(vector: Vec<&str>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    for one in vector.iter() {
        let mut array: Vec<Vec<String>> = Vec::new();
        let others: Vec<&str> = vector.clone().iter().filter(|v| *v == one).collect();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::combinator::combinate_strings;

    #[test]
    fn it_works() {
        assert_eq!(
            combinate_strings(vec!["A", "B"],),
            vec![vec!["A", "B"], vec!["B", "A"]]
        );
    }
}
