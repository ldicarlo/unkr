use crate::candidates;

use super::combinator;
use super::cryptors::get_decryptors;
use super::decrypt;
use super::encrypt;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn brute_force_decrypt(str: String, clues: Vec<String>) {
    let result = brute_force_strings(str, clues);
    println!("Result: {:?}", result);
}

pub fn internal_brute_force_decrypt(str: String, clues: Vec<String>) -> BTreeSet<Vec<(u8, u64)>> {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decryptors = get_decryptors();

    for i in combinator::combine_elements(decryptors.len().try_into().unwrap(), 4) {
        loop_decrypt(
            results_accumulator.clone(),
            vec![],
            i,
            vec![str.clone()],
            clues.clone(),
        );
    }

    let result: BTreeSet<Vec<(u8, u64)>> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn brute_force_strings(str: String, clues: Vec<String>) -> BTreeSet<Vec<(String, u64)>> {
    internal_brute_force_decrypt(str, clues)
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|(current_id, seed)| {
                    let (_, d_name, _, _, _) = get_decryptors()
                        .into_iter()
                        .find(|(id, _, _, _, _)| *id == *current_id)
                        .unwrap();
                    (d_name, *seed)
                })
                .collect()
        })
        .collect()
}

fn loop_decrypt(
    res_acc: Arc<Mutex<BTreeSet<Vec<(u8, u64)>>>>,
    acc: Vec<(u8, u64)>,
    mut to_use: Vec<u8>,
    strs: Vec<String>,
    clues: Vec<String>,
) {
    let local_arc = res_acc.clone();

    if let Some(current) = to_use.pop() {
        let (_id, _, seed, decrypt, _) = get_decryptors()
            .into_iter()
            .find(|(id, _, _, _, _)| *id == current)
            .unwrap();
        for s in 0..seed(strs.clone().len()) {
            let new_str = decrypt(strs.clone(), s);
            let mut current_acc = acc.clone();
            let current_to_use = to_use.clone();

            current_acc.push((current.clone(), s));
            let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

            if candidates.len() > 0 {
                println!("{:?}", candidates);
                local_arc
                    .clone()
                    .lock()
                    .unwrap()
                    .insert(current_acc.clone());
            }
            loop_decrypt(
                local_arc.clone(),
                current_acc,
                current_to_use,
                new_str.clone(),
                clues.clone(),
            );
        }
    }
}

pub fn print_encrypt(str: String, decryptors: Vec<String>) {
    encrypt::encrypt(vec![str], decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}

pub fn print_decrypt(str: String, decryptors: Vec<String>) {
    decrypt::decrypt(vec![str], decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            BTreeSet::new(),
            internal_brute_force_decrypt(
                "str".to_string(),
                vec![
                    "CLOCK".to_string(),
                    "BERLIN".to_string(),
                    "NORTH".to_string(),
                    "EAST".to_string(),
                ]
            )
        );
    }

    #[test]
    fn it_works_2() {
        let decryptors = get_decryptors();
        for (_, d, _, decrypt, encrypt) in decryptors.iter() {
            assert_eq!(
                decrypt(encrypt(vec!["SOME STRING 123 !".to_string()], 1), 1),
                vec!["SOME STRING 123 !"],
                "error with {}",
                &d
            )
        }
    }

    #[test]
    fn brute_force_1() {
        assert_eq!(
            true,
            // encrypt --string BERLIN -- caesar:10 atbash caesar:5
            brute_force_strings(
                "TQDJMH".to_string(),
                vec![
                    "CLOCK".to_string(),
                    "BERLIN".to_string(),
                    "NORTH".to_string(),
                    "EAST".to_string(),
                ]
            )
            .contains(&vec![
                ("caesar".to_string(), 10),
                ("atbash".to_string(), 0),
                ("caesar".to_string(), 5)
            ])
        );
    }
}
