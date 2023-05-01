pub fn get_alphabet() -> Vec<char> {
    vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
}

pub fn get_alphabet_prefixed() -> Vec<char> {
    vec![
        ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
}

pub fn vec_to_string(input: Vec<(char, char)>) -> String {
    input.into_iter().flat_map(|(a, b)| vec![a, b]).collect()
}

pub fn string_to_vec(input: String) -> Vec<(char, char)> {
    let even: Vec<char> = input.chars().into_iter().step_by(2).collect();
    let uneven: Vec<char> = input.chars().skip(1).into_iter().step_by(2).collect();

    even.into_iter().zip(uneven).collect()
}

pub fn char_mod(c: char, number: usize, order: bool) -> char {
    char_mod_custom_alphabet(c, number, order, vec![])
}

pub fn char_position(c: char, alphabet: Vec<char>) -> Option<usize> {
    alphabet.into_iter().position(|elem| elem == c)
}

pub fn char_position_base(c: char) -> Option<usize> {
    char_position(c, get_alphabet())
}

pub fn char_mod_custom_alphabet(
    c: char,
    number: usize,
    order: bool,
    custom_alphabet: Vec<char>,
) -> char {
    let alphabet = merge_custom_alphabet(custom_alphabet);
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

pub fn merge_custom_alphabet(primary: Vec<char>) -> Vec<char> {
    merge_alphabets(primary, get_alphabet())
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

    #[test]
    fn char_pos() {
        assert_eq!(super::char_position_base('D'), Some(3));
    }
}
