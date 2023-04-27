// https://cs.stackexchange.com/a/10321
pub fn change_base(input: Vec<u8>, before_base: u8, new_base: u8) -> Vec<u8> {
    let mut new_vec = Vec::new();
    let mut n = 0;
    input
        .into_iter()
        .rev()
        .for_each(|n| new_vec.push(n % new_base));
    new_vec
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(super::change_base(vec![1, 0], 2, 10), vec![2]);
        assert_eq!(super::change_base(vec![2], 10, 2), vec![1, 0]);
    }
}
