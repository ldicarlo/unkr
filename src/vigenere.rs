pub fn get_max_seed(text_length: u8) -> u64 {
    text_length.into()
}

pub fn encrypt(str: String, seed: u64) -> String {
    let (res, _) = str.split_at(seed.try_into().unwrap());
    res.to_string()
}

pub fn decrypt(str: String, _: u64) -> String {
    str
}
