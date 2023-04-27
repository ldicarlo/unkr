// https://cs.stackexchange.com/a/10321
pub fn increment(input: Vec<u8>, base: u8) -> Vec<u8> {
    let number = from_digits(input, base);
    to_digits(number + 1, base)
}

pub fn to_digits(mut input: u64, base: u8) -> Vec<u8> {
    let mut result = Vec::new();

    while input > 0 {
        result.insert(0, (input % base as u64) as u8);
        input = input / base as u64;
    }
    result
}

pub fn from_digits(input: Vec<u8>, base: u8) -> u64 {
    let mut n = 0;
    input
        .into_iter()
        .for_each(|d| n = n * base as u64 + d as u64);
    n
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(super::from_digits(vec![1, 0], 2), 2);
        assert_eq!(super::to_digits(2, 2), vec![1, 0]);
        assert_eq!(super::increment(vec![23, 24, 25], 26), vec![23, 25, 0]);
    }
}
