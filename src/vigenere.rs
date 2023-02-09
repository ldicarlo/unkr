use std::collections::HashSet;

pub fn get_max_seed(_: usize) -> u64 {
    3
}

pub fn encrypt(str: String, seed: u64) -> String {
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

pub fn decrypt(str: String, _: u64) -> String {
    str
}
