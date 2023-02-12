use super::parser;
pub fn encrypt(str: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    
decryptors
    
        .iter()
        .map(|str| parser::read_parameters(str.to_string()))
        .fold(str, |acc, (decryptor_name, seed)| {
          Vec::new()
           // current_encryptor(acc, seed)
        })
}