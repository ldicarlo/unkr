use crate::{
    atbash, caesar, cut, enigma, join,
    models::{BruteForceCryptor, BruteForceState, PermuteBruteForceState, VigenereBruteForceState},
    permute, reverse, swap, transpose, vigenere,
};

pub fn start_state(brute_force_cryptor: BruteForceCryptor) -> BruteForceState {
    match brute_force_cryptor {
        BruteForceCryptor::Vigenere(brute_force_args) => {
            BruteForceState::Vigenere(VigenereBruteForceState {
                brute_force_args,
                args: vigenere::init(),
            })
        }
        BruteForceCryptor::Cut => BruteForceState::Cut(transpose::init()),
        BruteForceCryptor::Caesar => BruteForceState::Caesar(caesar::init()),
        BruteForceCryptor::Transpose => BruteForceState::Transpose(transpose::init()),
        BruteForceCryptor::AtBash => BruteForceState::AtBash,
        BruteForceCryptor::Reverse => BruteForceState::Reverse,
        BruteForceCryptor::Swap => BruteForceState::Swap(swap::init()),
        BruteForceCryptor::Join => BruteForceState::Join,
        BruteForceCryptor::Permute(brute_force_args) => {
            BruteForceState::Permute(PermuteBruteForceState {
                brute_force_args,
                args: permute::init(),
            })
        }
        BruteForceCryptor::Enigma => BruteForceState::Enigma(enigma::init()),
    }
}

pub fn increase_state(bfs: BruteForceState, strs: Vec<String>) -> Option<BruteForceState> {
    match bfs {
        BruteForceState::Vigenere(state) => vigenere::next(state.clone()).map(|args| {
            BruteForceState::Vigenere(VigenereBruteForceState {
                args,
                brute_force_args: state.brute_force_args,
            })
        }),
        BruteForceState::Cut(args) => transpose::next(strs, args).map(|x| BruteForceState::Cut(x)),
        BruteForceState::Caesar(args) => caesar::next(args).map(|a| BruteForceState::Caesar(a)),
        BruteForceState::Transpose(args) => {
            transpose::next(strs, args).map(|x| BruteForceState::Transpose(x))
        }
        BruteForceState::AtBash => None,
        BruteForceState::Reverse => None,
        BruteForceState::Swap(swap_args) => {
            swap::next(swap_args, strs[0].len()).map(|s| BruteForceState::Swap(s))
        }
        BruteForceState::Join => None,
        BruteForceState::Permute(state) => permute::next(state.clone()).map(|args| {
            BruteForceState::Permute(PermuteBruteForceState {
                brute_force_args: state.brute_force_args,
                args,
            })
        }),
        BruteForceState::Enigma(state) => {
            enigma::next(state).map(|args| BruteForceState::Enigma(args))
        }
    }
}

pub fn apply_decrypt(bfs: BruteForceState, strings: Vec<String>) -> Vec<String> {
    let result = match bfs {
        BruteForceState::Vigenere(VigenereBruteForceState {
            args,
            brute_force_args: _,
        }) => vigenere::decrypt(strings.clone(), args),
        BruteForceState::Cut(args) => cut::encrypt(strings.clone(), args),
        BruteForceState::Caesar(args) => caesar::decrypt(strings.clone(), args),
        BruteForceState::Transpose(args) => transpose::decrypt(strings.clone(), args),
        BruteForceState::AtBash => atbash::decrypt(strings.clone()),
        BruteForceState::Reverse => reverse::decrypt(strings.clone()),
        BruteForceState::Swap(args) => swap::decrypt(strings.clone(), args),
        BruteForceState::Join => join::decrypt(strings.clone()),
        BruteForceState::Permute(PermuteBruteForceState {
            brute_force_args: _,
            args,
        }) => permute::decrypt(strings.clone(), args),
        BruteForceState::Enigma(args) => enigma::decrypt(strings.clone(), args),
    };
    if result == strings {
        vec![]
    } else {
        result
    }
}

pub fn get_name(bfs: &BruteForceCryptor) -> String {
    match bfs {
        // send args with it.
        BruteForceCryptor::Vigenere(_) => String::from("Vigenere"),
        BruteForceCryptor::Cut => String::from("Cut"),
        BruteForceCryptor::Caesar => String::from("Caesar"),
        BruteForceCryptor::Transpose => String::from("Transpose"),
        BruteForceCryptor::AtBash => String::from("AtBash"),
        BruteForceCryptor::Reverse => String::from("Reverse"),
        BruteForceCryptor::Swap => String::from("Swap"),
        BruteForceCryptor::Join => String::from("Join"),
        BruteForceCryptor::Permute(_) => String::from("Permute"),
        BruteForceCryptor::Enigma => String::from("Enigma"),
    }
}

pub fn loop_decrypt(
    acc: Option<String>,
    mut to_use: Vec<BruteForceCryptor>,
    strings: Vec<String>,
    clues: Vec<String>,
    candidates_sender: std::sync::mpsc::Sender<(Vec<String>, Vec<String>, String)>,
) {
    if let Some(current) = to_use.pop() {
        let mut bfs = start_state(current.clone());
        let current_acc = acc
            .clone()
            .map(|existing| existing + " " + &get_name(&current))
            .unwrap_or(get_name(&current));
        loop {
            let new_str = apply_decrypt(bfs.clone(), strings.clone());
            if new_str.len() > 0 {
                candidates_sender
                    .send((new_str.clone(), clues.clone(), current_acc.clone()))
                    .unwrap();
                loop_decrypt(
                    acc.clone(),
                    to_use.clone(),
                    strings.clone(),
                    clues.clone(),
                    candidates_sender.clone(),
                );
            }
            if let Some(next_is) = increase_state(bfs.clone(), strings.clone()) {
                bfs = next_is;
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
        assert_eq!(
            super::increase_state(super::BruteForceState::Join, vec![]),
            None
        );
    }
    #[test]
    fn vigenere_works() {
        assert_eq!(
            super::increase_state(
                super::increase_state(
                    super::BruteForceState::Vigenere(super::VigenereBruteForceState {
                        brute_force_args: crate::models::BruteForceVigenereArgs {
                            alphabet_depth: 1,
                            key_depth: 1
                        },
                        args: crate::models::VigenereArgs {
                            alphabet: String::from("Z"),
                            key: String::from("Y")
                        }
                    }),
                    vec![]
                )
                .unwrap(),
                vec![]
            ),
            None
        );
    }
}
