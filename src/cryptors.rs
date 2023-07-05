use crate::models::{BruteForceCryptor, BruteForcePermuteArgs, BruteForceVigenereArgs};

pub fn get_decryptors() -> Vec<BruteForceCryptor> {
    vec![
        BruteForceCryptor::Vigenere(BruteForceVigenereArgs {
            alphabet_depth: 1,
            key_depth: 2,
        }),
        BruteForceCryptor::Cut,
        BruteForceCryptor::Caesar,
        BruteForceCryptor::Transpose,
        BruteForceCryptor::AtBash,
        BruteForceCryptor::Reverse,
        BruteForceCryptor::Swap,
        BruteForceCryptor::Join,
        // IndexCrypt,
        BruteForceCryptor::Permute(BruteForcePermuteArgs {
            max_permutations: 2,
        }),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{
        atbash, caesar, cut, join,
        models::{self, BruteForceCryptor},
        permute, reverse, swap, transpose, vigenere,
    };

    use super::get_decryptors;

    #[test]
    fn it_works() {
        let strs = vec![
            String::from("ABCD"),
            String::from("EF"),
            String::from("GHIJ"),
            String::from("KLMNOP"),
            String::from("QRSTUVXYZ"),
        ];
        get_decryptors().into_iter().for_each(|name| match name {
            BruteForceCryptor::AtBash => {
                assert_eq!(atbash::decrypt(atbash::decrypt(strs.clone())), strs)
            }
            BruteForceCryptor::Caesar => {
                assert_eq!(
                    caesar::decrypt(
                        caesar::encrypt(strs.clone(), models::NumberArgs { number: 5 }),
                        models::NumberArgs { number: 5 }
                    ),
                    strs.clone()
                )
            }
            BruteForceCryptor::Reverse => {
                assert_eq!(
                    reverse::decrypt(reverse::decrypt(strs.clone())),
                    strs.clone()
                )
            }
            BruteForceCryptor::Transpose => {
                assert_eq!(
                    transpose::decrypt(
                        transpose::encrypt(strs.clone(), models::NumberArgs { number: 4 }),
                        models::NumberArgs { number: 4 }
                    )
                    .join(""),
                    strs.clone().join("")
                )
            }
            BruteForceCryptor::Vigenere(_) => {
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
            BruteForceCryptor::Cut => {
                assert_eq!(
                    cut::decrypt(cut::encrypt(strs.clone(), models::NumberArgs { number: 4 })),
                    join::decrypt(strs.clone())
                )
            }
            BruteForceCryptor::Join => {
                assert_eq!(
                    join::decrypt(join::decrypt(strs.clone())),
                    join::decrypt(strs.clone())
                )
            }
            BruteForceCryptor::Permute(_) => {
                assert_eq!(
                    permute::decrypt(
                        permute::decrypt(
                            strs.clone(),
                            models::PermuteArgs {
                                permutations: vec![('H', 'E'),].into_iter().collect(),
                                reversed_permutations: vec![('E', 'H')].into_iter().collect(),
                            }
                        ),
                        models::PermuteArgs {
                            permutations: vec![('H', 'E'),].into_iter().collect(),
                            reversed_permutations: vec![('E', 'H')].into_iter().collect(),
                        }
                    ),
                    strs.clone()
                )
            }
            BruteForceCryptor::Swap => {
                assert_eq!(
                    swap::decrypt(
                        swap::encrypt(
                            strs.clone(),
                            models::SwapArgs {
                                order: vec![4, 1, 0, 2, 3]
                            }
                        ),
                        models::SwapArgs {
                            order: vec![4, 1, 0, 2, 3]
                        }
                    ),
                    strs.clone()
                )
            }
            _ => todo!(),
        });
    }
}
