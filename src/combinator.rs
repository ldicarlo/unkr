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
///

type Combination = Vec<Vec<Step>>;

struct Step {
    decryptor: String,
    executions: u8,
}

pub fn get_combinations(vector: Vec<&str>, range: Vec<u8>) {}

pub fn combinate_strings(vector: Vec<&str>) -> Vec<Vec<String>> {
    vec![]
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
