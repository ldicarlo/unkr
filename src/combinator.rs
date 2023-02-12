use std::collections::HashSet;

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
/// - A,B,C
/// - A,C,B
/// - B,A,C
/// - B,C,A
/// - C,B,A
/// - C,A,B

pub fn combinate_strings(vector: Vec<u8>) -> std::collections::HashSet<Vec<u8>> {
    let mut result: HashSet<Vec<u8>> = HashSet::new();
    for one in vector.clone().into_iter() {
        for two in vector.clone().into_iter() {
            for three in vector.clone().into_iter() {
                result.insert(vec![one, two, three]);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests_combine_element {
    use super::*;

}

/// 2, 2 
/// -> [[0,0], [0,1], [1,0], [1,1]] 
/// -> 00, 01, 10, 11
/// -> 0, 1, 2, 3
pub fn combine_elements(elements_count: u8, picks:u8) -> std::collections::HashSet<Vec<u8>> {
    let result: HashSet<Vec<u8>> = HashSet::new();
    for pick in 0..picks  {
        
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut result = HashSet::new();

        result.insert(vec![1, 1, 2]);
        result.insert(vec![2, 2, 1]);
        result.insert(vec![2, 2, 2]);
        result.insert(vec![2, 1, 2]);
        result.insert(vec![1, 2, 2]);
        result.insert(vec![2, 1, 1]);
        result.insert(vec![1, 2, 1]);
        result.insert(vec![1, 1, 1]);

        assert_eq!(combinate_strings(vec![1, 2],), result);
    }

    #[test]
    fn it_works_2() {
        let mut result = HashSet::new();

        result.insert(vec![2, 1, 1]);
        result.insert(vec![2, 2, 3]);
        result.insert(vec![2, 3, 2]);
        result.insert(vec![2, 3, 3]);
        result.insert(vec![1, 1, 3]);
        result.insert(vec![2, 3, 1]);
        result.insert(vec![3, 1, 2]);
        result.insert(vec![2, 2, 2]);
        result.insert(vec![3, 2, 3]);
        result.insert(vec![2, 2, 1]);
        result.insert(vec![1, 3, 1]);
        result.insert(vec![3, 3, 2]);
        result.insert(vec![3, 3, 3]);
        result.insert(vec![1, 1, 2]);
        result.insert(vec![1, 2, 3]);
        result.insert(vec![3, 2, 2]);
        result.insert(vec![1, 2, 2]);
        result.insert(vec![1, 2, 1]);
        result.insert(vec![1, 3, 3]);
        result.insert(vec![2, 1, 2]);
        result.insert(vec![2, 1, 3]);
        result.insert(vec![3, 1, 3]);
        result.insert(vec![3, 2, 1]);
        result.insert(vec![3, 3, 1]);
        result.insert(vec![3, 1, 1]);
        result.insert(vec![1, 1, 1]);
        result.insert(vec![1, 3, 2]);

        assert_eq!(combinate_strings(vec![1, 2, 3],), result);
    }
}
