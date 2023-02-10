use std::collections::HashSet;

pub fn get_max_seed(_: usize) -> u64 {
    3
}

pub fn encrypt(str: Vec<String>, seed: u64) -> Vec<String> {
    let alphabet = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let words: HashSet<String> = vec![
        "palimpsest".to_string(),
        "abscissa".to_string(),
        "kryptos".to_string(),
    ]
    .into_iter()
    .collect();
    str
}

pub fn decrypt(str: Vec<String>, _: u64) -> Vec<String> {
    str
}
