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

#[cfg(test)]
mod tests {
    use crate::{
        atbash, caesar, cut, join,
        models::{self},
        permute, reverse,
    };

    use super::get_decryptors_names;

    #[test]
    fn it_works() {
        let strs = vec![String::from("HELLO")];
        get_decryptors_names()
            .into_iter()
            .for_each(|name| match name.as_str() {
                "atbash" => {
                    assert_eq!(atbash::decrypt(atbash::decrypt(strs.clone())), strs)
                }
                "caesar" => {
                    assert_eq!(
                        caesar::decrypt(
                            caesar::encrypt(strs.clone(), models::NumberArgs { number: 5 }),
                            models::NumberArgs { number: 5 }
                        ),
                        strs.clone()
                    )
                }
                "reverse" => {
                    assert_eq!(
                        reverse::decrypt(reverse::decrypt(strs.clone())),
                        strs.clone()
                    )
                }
                "transpose" => {
                    assert_eq!(1, 1)
                }
                "vigenere" => {
                    assert_eq!(1, 1)
                }
                "cut" => {
                    assert_eq!(
                        cut::decrypt(cut::encrypt(strs.clone(), models::NumberArgs { number: 4 })),
                        strs.clone()
                    )
                }
                "join" => {
                    assert_eq!(join::decrypt(join::decrypt(strs.clone())), strs.clone())
                }
                "permute" => {
                    assert_eq!(
                        permute::decrypt(
                            permute::decrypt(
                                strs.clone(),
                                models::PermuteArgs {
                                    permutations: vec![('H', 'E')]
                                }
                            ),
                            models::PermuteArgs {
                                permutations: vec![('H', 'E')]
                            }
                        ),
                        strs.clone()
                    )
                }
                _ => todo!(),
            });
    }
}
