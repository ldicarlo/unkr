use std::collections::BTreeMap;

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

pub fn pairs_to_vec<A>(input: BTreeMap<A, A>) -> Vec<A> {
    input.into_iter().flat_map(|(a, b)| vec![a, b]).collect()
}

// better perf like that.
pub fn vec_to_pairs<A: Clone>(input: &Vec<A>) -> Vec<(A, A)> {
    let mut result = Vec::new();

    for elem in (1..input.len()).step_by(2) {
        result.push((input[elem - 1].clone(), input[elem].clone()));
    }

    result
}

pub fn char_mod(c: char, number: usize, order: bool) -> char {
    char_mod_custom_alphabet(c, number, order, vec![])
}

pub fn char_position(c: char, alphabet: Vec<char>) -> Option<usize> {
    alphabet.into_iter().position(|elem| elem == c)
}

pub fn char_position_base(c: char) -> usize {
    (c as u32 - 65) as usize
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
        assert_eq!(super::char_position_base('D'), 3);
    }

    #[test]
    fn char_pos_tests() {
        assert_eq!(65, 'A' as u32);
        assert_eq!(66, String::from("ABCD").chars().nth(1).unwrap() as u32);
    }
}
