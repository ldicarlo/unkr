use std::collections::HashSet;

pub fn combine_elements(elements_count: u8, picks: u8) -> std::collections::HashSet<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for pick in 1..(picks + 1) {
        println!("PICK {}", pick);
        result.append(&mut generate_elements(vec![], vec![], elements_count, pick));
    }

    result.into_iter().collect::<HashSet<Vec<u8>>>()
}

fn generate_elements(
    mut acc: Vec<Vec<u8>>,
    current: Vec<u8>,
    elements_count: u8,
    picks: u8,
) -> Vec<Vec<u8>> {
    if picks > 0 {
        for element in 0..elements_count {
            let mut new_current = current.clone();
            new_current.push(element);
            acc.append(&mut generate_elements(
                acc.clone(),
                new_current,
                elements_count,
                picks - 1,
            ));
        }
        acc
    } else {
        vec![current]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        assert_eq!(
            combine_elements(3, 3),
            vec![
                vec![0],
                vec![1],
                vec![2],
                vec![0, 0],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2],
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 0, 2],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![0, 1, 2],
                vec![0, 2, 0],
                vec![0, 2, 1],
                vec![0, 2, 2],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 0, 2],
                vec![1, 1, 0],
                vec![1, 1, 1],
                vec![1, 1, 2],
                vec![1, 2, 0],
                vec![1, 2, 1],
                vec![1, 2, 2],
                vec![2, 0, 0],
                vec![2, 0, 1],
                vec![2, 0, 2],
                vec![2, 1, 0],
                vec![2, 1, 1],
                vec![2, 1, 2],
                vec![2, 2, 0],
                vec![2, 2, 1],
                vec![2, 2, 2],
            ]
            .into_iter()
            .collect::<HashSet<Vec<u8>>>()
        );
    }
}
