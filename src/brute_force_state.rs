use crate::{
    caesar,
    models::{BruteForceCryptor, BruteForceState, PermuteBruteForceState, VigenereBruteForceState},
    permute, vigenere,
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
        BruteForceCryptor::Permute(brute_force_args) => {
            BruteForceState::Permute(PermuteBruteForceState {
                brute_force_args,
                args: permute::init(),
            })
        }
        BruteForceCryptor::Enigma => todo!(),
    }
}

pub fn increase_state(bfs: BruteForceState) -> Option<BruteForceState> {
    println!("{:?}", bfs);
    match bfs {
        BruteForceState::Vigenere(state) => vigenere::next(state.clone()).map(|args| {
            BruteForceState::Vigenere(VigenereBruteForceState {
                args,
                brute_force_args: state.brute_force_args,
            })
        }),
        BruteForceState::Cut(args) => todo!(),
        BruteForceState::Caesar(_) => todo!(),
        BruteForceState::Transpose(_) => todo!(),
        BruteForceState::AtBash => None,
        BruteForceState::Reverse => None,
        BruteForceState::Swap(_) => todo!(),
        BruteForceState::Join => None,
        BruteForceState::Permute(state) => permute::next(state.clone()).map(|args| {
            BruteForceState::Permute(PermuteBruteForceState {
                brute_force_args: state.brute_force_args,
                args,
            })
        }),
        BruteForceState::Enigma(_) => None,
    }
}

pub fn apply_decrypt(bfs: BruteForceState, strings: Vec<String>) -> Vec<String> {
    match bfs {
        BruteForceState::Vigenere(_) => todo!(),
        BruteForceState::Cut(_) => todo!(),
        BruteForceState::Caesar(_) => todo!(),
        BruteForceState::Transpose(_) => todo!(),
        BruteForceState::AtBash => todo!(),
        BruteForceState::Reverse => todo!(),
        BruteForceState::Swap(_) => todo!(),
        BruteForceState::Join => todo!(),
        BruteForceState::Permute(PermuteBruteForceState {
            brute_force_args: _,
            args,
        }) => permute::decrypt(strings, args),
        BruteForceState::Enigma(_) => todo!(),
    }
}

fn loop_decrypt(
    acc: Option<String>,
    mut to_use: Vec<BruteForceState>,
    strings: Vec<String>,
    clues: Vec<String>,
    candidates_sender: std::sync::mpsc::Sender<(Vec<String>, Vec<String>, String)>,
) {
    if let Some(current) = to_use.pop() {
        let mut next = current;
        loop {
            let new_str = apply_decrypt(next.clone(), strings.clone());
            let current_acc = acc
            .clone()
            .map(|existing| existing + " " + &cryptor_str.clone())
            .unwrap_or(cryptor_str.clone());
        candidates_sender
            .send((new_str.clone(), clues.clone(), current_acc.clone()))
            .unwrap();

        Some(current_acc)
            if let Some(next_is) = increase_state(next.clone()) {
                next = next_is;
                continue;
            } else {
                break;
            }
        }
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
