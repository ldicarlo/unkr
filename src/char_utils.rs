pub fn char_mod(c: char, number: usize) -> char {
    let alphabet = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    char_position(c)
        .and_then(|index| alphabet.get((index + number) % 26))
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
