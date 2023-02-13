pub fn get_alphabet() -> Vec<char> {
    vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
}

pub fn char_mod(c: char, number: usize, order: bool) -> char {
    char_mod_custom_alphabet(c, number, order, vec![])
}

pub fn char_position(c: char, alphabet: Vec<char>) -> Option<usize> {
    alphabet.binary_search(&c).ok()
}

pub fn char_mod_custom_alphabet(
    c: char,
    number: usize,
    order: bool,
    custom_alphabet: Vec<char>,
) -> char {
    let alphabet = merge_alphabets(custom_alphabet, get_alphabet());
    char_position(c, alphabet.clone())
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

pub fn merge_alphabets(mut primary: Vec<char>, secondary: Vec<char>) -> Vec<char> {
    for c in secondary {
        if !primary.contains(&c) {
            primary.push(c);
        }
    }
    primary
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(super::char_mod('A', 4, false), 'W');
    }

    #[test]
    fn alphabet() {
        assert_eq!(
            super::merge_alphabets(vec!['B', 'C'], vec!['A', 'B', 'C', 'D']),
            vec!['B', 'C', 'A', 'D']
        );
    }
}
