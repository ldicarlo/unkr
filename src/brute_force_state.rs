use crate::{
    caesar, enigma,
    mapper::cryptor_base_from_cryptor,
    models::{
        BruteForceCryptor, BruteForceState, Cryptor, PermuteBruteForceState,
        VigenereBruteForceState,
    },
    permute, swap, transpose, vigenere,
};
use crossbeam::channel::Sender;
use std::collections::VecDeque;

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
        BruteForceCryptor::Reuse(arg) => BruteForceState::Reuse(arg),
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
        BruteForceState::Reuse(_) => None,
    }
}

pub fn apply_decrypt(
    bfs: BruteForceState,
    strings: Vec<String>,
    current_cryptors: Vec<Cryptor>,
) -> Vec<String> {
    sub_apply_decrypt(get_cryptor(&bfs, current_cryptors), strings)
}

fn sub_apply_decrypt(cryptor: Cryptor, strings: Vec<String>) -> Vec<String> {
    let result = match cryptor {
        Cryptor::Vigenere(args) => vigenere::decrypt(strings.clone(), args),
        Cryptor::Cut(args) => todo!(),
        Cryptor::Caesar(args) => todo!(),
        Cryptor::Transpose(args) => todo!(),
        Cryptor::AtBash => todo!(),
        Cryptor::Reverse => todo!(),
        Cryptor::Swap(args) => todo!(),
        Cryptor::Join => todo!(),
        Cryptor::Colors(args) => todo!(),
        //Cryptor::IndexCrypt(_) => todo!(),
        Cryptor::Permute(args) => permute::decrypt(strings.clone(), args),
        Cryptor::Enigma(args) => enigma::decrypt(strings.clone(), args),
    };
    if result == strings {
        vec![]
    } else {
        result
    }
}

pub fn get_cryptor(bfs: &BruteForceState, previous_cryptors: Vec<Cryptor>) -> Cryptor {
    match bfs {
        BruteForceState::Vigenere(VigenereBruteForceState {
            brute_force_args: _,
            args,
        }) => Cryptor::Vigenere(args.clone()),
        BruteForceState::Cut(args) => Cryptor::Cut(args.clone()),
        BruteForceState::Caesar(args) => Cryptor::Caesar(args.clone()),
        BruteForceState::Transpose(args) => Cryptor::Transpose(args.clone()),
        BruteForceState::AtBash => Cryptor::AtBash,
        BruteForceState::Reverse => Cryptor::Reverse,
        BruteForceState::Swap(args) => Cryptor::Swap(args.clone()),
        BruteForceState::Join => Cryptor::Join,
        BruteForceState::Permute(PermuteBruteForceState {
            brute_force_args: _,
            args,
        }) => Cryptor::Permute(args.clone()),
        BruteForceState::Enigma(args) => Cryptor::Enigma(args.clone()),
        BruteForceState::Reuse(arg) => previous_cryptors
            .into_iter()
            .filter(|cr| cryptor_base_from_cryptor(&cr) == arg)
            .nth(0)
            .unwrap(),
    }
}

pub fn loop_decrypt(
    acc: Vec<Cryptor>,
    mut to_use: VecDeque<BruteForceCryptor>,
    strings: Vec<String>,
    clues: Vec<String>,
    candidates_sender: Sender<(Vec<String>, Vec<String>, Vec<Cryptor>)>,
) {
    if let Some(current) = to_use.pop_front() {
        let mut bfs = start_state(current.clone());
        let mut current_acc = acc.clone();
        current_acc.push(get_cryptor(&bfs, current_acc.clone()));
        loop {
            let new_str = apply_decrypt(bfs.clone(), strings.clone(), vec![]);
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
