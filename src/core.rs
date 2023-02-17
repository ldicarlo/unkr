use crate::candidates;
use crate::cryptors;

use super::combinator;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors: Vec<String>,
    threads: u8,
) {
    let result = brute_force_strings(str, clues, steps, decryptors, threads);
    println!("Result: {:?}", result);
}

fn multi_thread() {}

pub fn internal_brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors_filtered: Vec<String>,
    threads: u8,
) -> BTreeSet<Vec<(u8, u64)>> {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decryptors = cryptors::filter_decryptors(decryptors_filtered.clone());
    let combinations = combinator::combine_elements(decryptors.len().try_into().unwrap(), steps);
    println!("TOTAL: {}", combinations.len());

    // add a mpsc here https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html

    let size_by_thread: usize = combinations.len() / (threads as usize);
    for t in 0..threads {
        for (i, vec) in combinations.iter().enumerate() {
            if i % 1 == 0 {
                println!("i: {}", i);
            }
            loop_decrypt(
                results_accumulator.clone(),
                vec![],
                vec.clone(),
                vec![str.clone()],
                clues.clone(),
                decryptors_filtered.clone(),
            );
        }
    }

    let result: BTreeSet<Vec<(u8, u64)>> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn brute_force_strings(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors_filtered: Vec<String>,
    threads: u8,
) -> BTreeSet<Vec<(String, u64)>> {
    internal_brute_force_decrypt(str, clues, steps, decryptors_filtered.clone(), threads)
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|(current_id, seed)| {
                    let (d_name, _, _, _) =
                        cryptors::filter_decryptors(decryptors_filtered.clone())
                            .into_iter()
                            .nth((*current_id).into())
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
    decryptors_filtered: Vec<String>,
) {
    let local_arc = res_acc.clone();

    if let Some(current) = to_use.pop() {
        let (_, seed, decrypt, _) = cryptors::filter_decryptors(decryptors_filtered.clone())
            .into_iter()
            .nth(current.into())
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
                decryptors_filtered.clone(),
            );
        }
    }
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
                ],
                2,
                vec![String::from("caesar"), String::from("atbash")],
                1
            )
        );
    }

    #[test]
    fn it_works_2() {
        let decryptors = cryptors::filter_decryptors(vec![]);
        for (d, _, decrypt, encrypt) in decryptors.iter() {
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
            vec![
                vec![("caesar".to_string(), 5), ("atbash".to_string(), 0),],
                vec![("atbash".to_string(), 0), ("caesar".to_string(), 21),]
            ]
            .into_iter()
            .collect::<BTreeSet<Vec<(std::string::String, u64)>>>(),
            // encrypt --string BERLIN -- caesar:10 atbash caesar:5
            brute_force_strings(
                "TQDJMH".to_string(),
                vec![
                    "CLOCK".to_string(),
                    "BERLIN".to_string(),
                    "NORTH".to_string(),
                    "EAST".to_string(),
                ],
                2,
                vec![String::from("caesar"), String::from("atbash")],
                1
            )
        );
    }
}
