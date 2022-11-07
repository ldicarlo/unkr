use std::collections::HashMap;

pub fn get_max_seed() -> u64 {
    1
}

pub fn decrypt(str: String, _seed: u64) -> String {
    let letters: HashMap<char, char> = vec![
        ('A', 'Z'),
        ('B', 'Y'),
        ('C', 'X'),
        ('D', 'W'),
        ('E', 'V'),
        ('F', 'U'),
        ('G', 'T'),
        ('H', 'S'),
        ('I', 'R'),
        ('J', 'Q'),
        ('K', 'P'),
        ('L', 'O'),
        ('M', 'N'),
        ('N', 'M'),
        ('O', 'L'),
        ('P', 'K'),
        ('Q', 'J'),
        ('R', 'I'),
        ('S', 'H'),
        ('T', 'G'),
        ('U', 'F'),
        ('V', 'E'),
        ('W', 'D'),
        ('X', 'C'),
        ('Y', 'B'),
        ('Z', 'A'),
    ]
    .into_iter()
    .collect();
    str.chars()
        .into_iter()
        .map(|c| {
            let res = c.clone();
            let maybe_char = letters.get(&c);
            match maybe_char {
                Some(found) => *found,
                None => res,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(super::decrypt("HELLO45?".to_string(), 1), "SVOOL45?");
    }
    #[test]
    fn it_works_2() {
        assert_eq!(super::decrypt("BERLIN".to_string(), 1), "YVIORM");
    }
}
