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
        "swap".to_string(),
        //"indexcrypt".to_string(),
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
    use crate::{atbash, caesar, cut, join, models, permute, reverse, vigenere, swap};

    use super::get_decryptors_names;

    #[test]
    fn it_works() {
        let strs = vec![String::from("HELLO"),String::from("TO"),String::from("THE"),String::from("WORLD"),String::from("HELLOWORLD")];
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
                    assert_eq!(
                        vigenere::decrypt(
                            vigenere::encrypt(
                                strs.clone(),
                                models::VigenereArgs {
                                    alphabet: String::from("FIRST"),
                                    key: String::from("HELLO")
                                }
                            ),
                            models::VigenereArgs {
                                alphabet: String::from("FIRST"),
                                key: String::from("HELLO")
                            }
                        ),
                        strs.clone()
                    )
                }
                "cut" => {
                    assert_eq!(
                        cut::decrypt(cut::encrypt(strs.clone(), models::NumberArgs { number: 4 })),
                        join::decrypt(strs.clone())
                    )
                }
                "join" => {
                    assert_eq!(join::decrypt(join::decrypt(strs.clone())), join::decrypt(strs.clone()))
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
                "swap" => {

                  assert_eq!(
                    swap::decrypt(
                        swap::encrypt(
                            strs.clone(),
                            models::SwapArgs {
                                order: vec![4,1,0,2,3]
                            }
                        ),
                        models::SwapArgs {
                          order: vec![4,1,0,2,3]
                      }
                    ),
                    strs.clone()
                )
                }
                _ => todo!(),
            });
    }
}
