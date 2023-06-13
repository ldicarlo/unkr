use crate::{
    caesar,
    models::{BruteForceCryptor, BruteForceState, VigenereBruteForceState},
    vigenere,
};

pub fn start_state(brute_force_cryptor: BruteForceCryptor) -> BruteForceState {
    match brute_force_cryptor {
        BruteForceCryptor::Vigenere(brute_force_args) => {
            BruteForceState::Vigenere(VigenereBruteForceState {
                brute_force_args,
                args: vigenere::init(),
            })
        }
        BruteForceCryptor::Cut => todo!(),
        BruteForceCryptor::Caesar => BruteForceState::Caesar(caesar::init()),
        BruteForceCryptor::Transpose => todo!(),
        BruteForceCryptor::AtBash => todo!(),
        BruteForceCryptor::Reverse => todo!(),
        BruteForceCryptor::Swap => todo!(),
        BruteForceCryptor::Join => BruteForceState::Join,
        BruteForceCryptor::Permute(_) => todo!(),
        BruteForceCryptor::Enigma => todo!(),
    }
}

pub fn increase_state(bfs: BruteForceState) -> Option<BruteForceState> {
    match bfs {
        BruteForceState::Vigenere(state) => vigenere::next(state.clone()).map(|args| {
            {
                BruteForceState::Vigenere(VigenereBruteForceState {
                    args,
                    brute_force_args: state.brute_force_args,
                })
            }
        }),
        BruteForceState::Cut(args) => todo!(),
        BruteForceState::Caesar(_) => todo!(),
        BruteForceState::Transpose(_) => todo!(),
        BruteForceState::AtBash => None,
        BruteForceState::Reverse => None,
        BruteForceState::Swap(_) => todo!(),
        BruteForceState::Join => None,
        BruteForceState::Permute(_) => todo!(),
        BruteForceState::Enigma(_) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::increase_state(super::BruteForceState::Join), None);
    }
    #[test]
    fn vigenere_works() {
        assert_eq!(
            super::increase_state(
                super::increase_state(super::BruteForceState::Vigenere(
                    super::VigenereBruteForceState {
                        brute_force_args: crate::models::BruteForceVigenereArgs {
                            alphabet_depth: 1,
                            key_depth: 1
                        },
                        args: crate::models::VigenereArgs {
                            alphabet: String::from("Z"),
                            key: String::from("Y")
                        }
                    }
                ))
                .unwrap()
            ),
            None
        );
    }
}
