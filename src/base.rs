// https://cs.stackexchange.com/a/10321
pub fn increment(input: Vec<u8>, base: usize) -> Vec<u8> {
    let mut number = from_digits(input, base);

    let mut result = to_digits(number + 1, base);
    while result.clone().into_iter().any(|a| a == 0) {
        // !?
        number = number + 1;
        // !?
        result = to_digits(number + 1, base);
    }

    result
}

pub fn to_digits(mut input: u64, base: usize) -> Vec<u8> {
    let mut result = Vec::new();

    while input > 0 {
        result.insert(0, (input % base as u64) as u8);
        input = input / base as u64;
    }
    result
}

pub fn from_digits(input: Vec<u8>, base: usize) -> u64 {
    let mut n = 0;
    input
        .into_iter()
        .for_each(|d| n = n * base as u64 + d as u64);
    n
}

pub fn increment_with_bases(input: Vec<u8>, bases: Vec<usize>) -> Vec<u8> {
    let mut result = input.clone();
    let input_len = input.len();
    let bases_len = bases.len();
    let mut last_inc = (result[input_len - 1] + 1) % (bases[bases_len - 1] as u8);
    if last_inc == 0 {
        last_inc = 1;
        let mut inc = true;
        let mut i = 2;
        while inc {
            let mut next = (result[input_len - i] + 1) % (bases[bases_len - i] as u8);
            if next == 0 {
                next = 1;
                inc = true;
            } else {
                inc = false;
            }
            result[input_len - i] = next;
            i = i + 1;
        }
    }
    result[input_len - 1] = last_inc;

    result
}

fn from_digits_and_bases(input: Vec<u8>, bases: Vec<usize>) -> u64 {
    let mut n = 0;
    let base_len = bases.len();
    input
        .into_iter()
        .enumerate()
        .for_each(|(i, d)| n = n * bases[base_len - i - 1] as u64 + d as u64);
    n
}

fn to_digits_and_bases(mut input: u64, bases: Vec<usize>) -> Vec<u8> {
    let mut result = Vec::new();
    let base_len = bases.len();
    let mut i = 0;

    while input > 0 {
        i = i + 1;
        result.insert(0, (input % bases[base_len - i] as u64) as u8);
        input = input / bases[base_len - i] as u64;
    }
    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(super::from_digits(vec![1, 0], 2), 2);
        assert_eq!(super::to_digits(2, 2), vec![1, 0]);
        assert_eq!(super::increment(vec![23, 24, 25], 26), vec![23, 25, 1]);
        assert_eq!(super::increment(vec![23, 25, 25], 26), vec![24, 1, 1]);
    }
    #[test]
    fn it_works_with_bases() {
        // A, A => A, B
        assert_eq!(super::to_digits_and_bases(3, vec![2, 2]), vec![1, 1]);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            super::increment_with_bases(vec![23, 24, 25], vec![26, 26, 26]),
            vec![23, 25, 1]
        );
        assert_eq!(
            super::increment_with_bases(vec![23, 25, 25], vec![26, 26, 26]),
            vec![24, 1, 1]
        );
    }

    #[test]
    fn multiple_bases_works() {
        assert_eq!(
            super::increment_with_bases(
                // B, C, Z
                vec![2, 3, 26],
                vec![27, 4, 27]
            ),
            // C,A,A
            vec![3, 1, 1]
        );
    }

    #[test]
    fn multiple_bases_works_2() {
        assert_eq!(
            super::increment_with_bases(vec![1, 1, 4, 2, 3, 3, 13], vec![1, 4, 27, 4, 27, 4, 27]),
            vec![1, 1, 4, 2, 3, 3, 14],
        );
    }

    #[test]
    fn multiple_bases_works_3() {
        assert_eq!(
            super::increment_with_bases(vec![1, 1, 26, 3, 26, 3, 26], vec![1, 4, 27, 4, 27, 4, 27]),
            vec![1, 2, 1, 1, 1, 1, 1],
        );
    }

    #[test]
    fn to_digits_and_bases_works() {
        assert_eq!(
            super::to_digits_and_bases(
                // B, C, Z
                60292,
                vec![1, 4, 27, 4, 27, 4, 27]
            ),
            // C,A,A
            vec![1, 1, 4, 2, 3, 3, 14],
        );
    }
}
