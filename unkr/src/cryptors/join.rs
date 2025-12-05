use crate::models;

pub fn skip_if_previous_in() -> Vec<models::BruteForceCryptor> {
    vec![
        models::BruteForceCryptor::Join,
        models::BruteForceCryptor::Cut,
    ]
}

pub fn decrypt(strs: Vec<String>) -> Vec<String> {
    vec![strs.join("")]
}
