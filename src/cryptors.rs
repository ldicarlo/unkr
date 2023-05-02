fn get_decryptors_names() -> Vec<String> {
    vec![
        "atbash".to_string(),
        "caesar".to_string(),
        "reverse".to_string(),
        "transpose".to_string(),
        "vigenere".to_string(),
        "cut".to_string(),
        "join".to_string(),
        "permute".to_string(),
    ]
}

pub fn filter_decryptors(decryptors_filtered: Vec<String>) -> Vec<String> {
    if decryptors_filtered.is_empty() {
        get_decryptors_names()
    } else {
        get_decryptors_names()
            .into_iter()
            .filter(|decryptor| decryptors_filtered.contains(decryptor))
            .collect()
    }
}
