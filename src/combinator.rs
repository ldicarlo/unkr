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

/// 2, 2
/// -> [[0,0], [0,1], [1,0], [1,1]]
/// -> 0, 1, 2, 0
pub fn combine_elements(elements_count: u8, picks: u8) -> std::collections::HashSet<Vec<u8>> {
    generate_elements(vec![], vec![], elements_count, picks)
        .into_iter()
        .collect::<HashSet<Vec<u8>>>()
}

fn generate_elements(
    mut acc: Vec<Vec<u8>>,
    current: Vec<u8>,
    elements_count: u8,
    picks: u8,
) -> Vec<Vec<u8>> {
    if picks > 0 {
        let mut current_acc = vec![];
        for element in 0..elements_count {
            let mut new_current = current.clone();
            new_current.push(element);
            current_acc.append(&mut generate_elements(
                acc.clone(),
                new_current,
                elements_count,
                picks - 1,
            ));
        }
        acc.append(&mut current_acc);
        acc
    } else {
        vec![current]
    }
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
        result.insert(vec![2, 2, 0]);
        result.insert(vec![2, 0, 2]);
        result.insert(vec![2, 0, 0]);
        result.insert(vec![1, 1, 0]);
        result.insert(vec![2, 0, 1]);
        result.insert(vec![0, 1, 2]);
        result.insert(vec![2, 2, 2]);
        result.insert(vec![0, 2, 0]);
        result.insert(vec![2, 2, 1]);
        result.insert(vec![1, 0, 1]);
        result.insert(vec![0, 0, 2]);
        result.insert(vec![0, 0, 0]);
        result.insert(vec![1, 1, 2]);
        result.insert(vec![1, 2, 0]);
        result.insert(vec![0, 2, 2]);
        result.insert(vec![1, 2, 2]);
        result.insert(vec![1, 2, 1]);
        result.insert(vec![1, 0, 0]);
        result.insert(vec![2, 1, 2]);
        result.insert(vec![2, 1, 0]);
        result.insert(vec![0, 1, 0]);
        result.insert(vec![0, 2, 1]);
        result.insert(vec![0, 0, 1]);
        result.insert(vec![0, 1, 1]);
        result.insert(vec![1, 1, 1]);
        result.insert(vec![1, 0, 2]);

        assert_eq!(combine_elements(0, 0), result);
    }
}
