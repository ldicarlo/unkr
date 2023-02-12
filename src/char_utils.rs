pub fn char_mod(c: char, number: usize, order: bool) -> char {
    let alphabet = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    char_position(c)
        .and_then(|index| {
            alphabet.get(
                (if order {
                    index + number
                } else {
                    // bad fix
                    26 + index - number
                }) % 26,
            )
        })
        .map(|ch| *ch)
        .unwrap_or(c)
}

pub fn char_position(c: char) -> Option<usize> {
    let alphabet = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    alphabet.binary_search(&c).ok()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(super::char_mod('A', 4, false), 'W');
    }
}
