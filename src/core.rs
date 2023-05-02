use super::combinator;
use crate::atbash;
use crate::caesar;
use crate::candidates;
use crate::cryptors;
use crate::cut;
use crate::join;
use crate::models::PermuteArgs;
use crate::permute;
use crate::reverse;
use crate::transpose;
use crate::vigenere;

use rand::prelude::SliceRandom;
use std::collections::BTreeSet;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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

    let count_by_thread: usize = combinations.len() / (threads as usize);
    println!(
        "TOTAL: {}, threads: {}, count per thread: {}, decryptors {:?}",
        combinations.len(),
        threads,
        count_by_thread,
        decryptors_filtered
    );
    let mut vec_combinations = combinations.into_iter().collect::<Vec<Vec<u8>>>();

    vec_combinations.shuffle(&mut rand::thread_rng());

    let (sender, receiver) = channel();
    for t in 0..threads {
        let local_combinations = if t == threads - 1 {
            vec_combinations.clone()
        } else {
            vec_combinations.split_off(vec_combinations.len() - count_by_thread)
        };
        let local_sender = sender.clone();
        let local_results_accumulator = results_accumulator.clone();
        let local_str = str.clone();
        let local_clues = clues.clone();
        let local_decryptors = decryptors_filtered.clone();
        println!(
            "Start thread {} with {} combinations",
            t,
            local_combinations.len()
        );
        thread::spawn(move || {
            local_sender
                .send(threaded_function(
                    t,
                    local_combinations,
                    local_results_accumulator,
                    local_str,
                    local_clues,
                    local_decryptors,
                ))
                .unwrap();
        });
    }
    for t in 0..threads {
        println!("THREAD {} finished({})", t, receiver.recv().unwrap());
    }

    let result: BTreeSet<Vec<(u8, u64)>> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn threaded_function(
    thread_number: u8,
    combinations: Vec<Vec<u8>>,
    results_accumulator: Arc<Mutex<BTreeSet<Vec<(u8, u64)>>>>,
    str: String,
    clues: Vec<String>,
    decryptors_filtered: Vec<String>,
) -> bool {
    // let cache = BTreeSet::new();
    for (i, vec) in combinations.iter().enumerate() {
        if i % 10 == 0 {
            println!("THREAD {}\tcombination: {}", thread_number, i);
        }
        loop_decrypt(
            results_accumulator.clone(),
            vec![],
            vec.clone(),
            vec![str.clone()],
            clues.clone(),
            // cache.clone(),
            decryptors_filtered.clone(),
        );
    }
    true
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
                    let d_name = cryptors::filter_decryptors(decryptors_filtered.clone())
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
    // mut cache: BTreeSet<(Vec<String>, Vec<u8>, u64)>,
    decryptors_filtered: Vec<String>,
) {
    //println!("{:?} {:?} {:?}", acc, to_use, strs);
    if let Some(current) = to_use.pop() {
        let cryptor_name = cryptors::filter_decryptors(decryptors_filtered.clone())
            .into_iter()
            .nth(current.into())
            .unwrap();

        match cryptor_name.as_str() {
            // make that an enum
            "atbash" => {
                let new_str = atbash::decrypt_from_args(strs);
                let mut current_acc = acc.clone();
                let current_to_use = to_use.clone();
                current_acc.push((current.clone(), 1));
                let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                if candidates.len() > 0 {
                    let local_arc = res_acc.clone();
                    local_arc.lock().unwrap().insert(current_acc.clone());
                }

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    current_to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                );
            }
            "caesar" => {
                for s in 0..26 {
                    let new_str = caesar::decrypt(strs.clone(), s);
                    let mut current_acc = acc.clone();
                    let current_to_use = to_use.clone();
                    current_acc.push((current.clone(), s));
                    let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                    if candidates.len() > 0 {
                        let local_arc = res_acc.clone();
                        local_arc.lock().unwrap().insert(current_acc.clone());
                    }

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        current_to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                    );
                }
            }
            "reverse" => {
                let new_str = reverse::decrypt_from_args(strs);
                let mut current_acc = acc.clone();
                let current_to_use = to_use.clone();
                current_acc.push((current.clone(), 1));
                let candidates =
                    candidates::find_and_print_candidates(new_str.clone(), clues.clone());

                if candidates.len() > 0 {
                    let local_arc = res_acc.clone();
                    local_arc.lock().unwrap().insert(current_acc.clone());
                }

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    current_to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                );
            }
            "transpose" => {
                for s in 0..strs.first().map(|s| s.len()).unwrap_or(0) {
                    let new_str = transpose::decrypt(strs.clone(), s as u64);
                    let mut current_acc = acc.clone();
                    let current_to_use = to_use.clone();
                    current_acc.push((current.clone(), s as u64));
                    let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                    if candidates.len() > 0 {
                        let local_arc = res_acc.clone();
                        local_arc.lock().unwrap().insert(current_acc.clone());
                    }

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        current_to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                    );
                }
            }
            "vigenere" => {
                for s in 0..vigenere::get_max_seed() {
                    let new_str = vigenere::decrypt(strs.clone(), s);
                    let mut current_acc = acc.clone();
                    let current_to_use = to_use.clone();
                    current_acc.push((current.clone(), s as u64));
                    let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                    if candidates.len() > 0 {
                        let local_arc = res_acc.clone();
                        local_arc.lock().unwrap().insert(current_acc.clone());
                    }

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        current_to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                    );
                }
            }
            "cut" => {
                for s in 0..strs.first().map(|s| s.len()).unwrap_or(0) {
                    let new_str = cut::encrypt(strs.clone(), s as u64);
                    let mut current_acc = acc.clone();
                    let current_to_use = to_use.clone();
                    current_acc.push((current.clone(), s as u64));
                    let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                    if candidates.len() > 0 {
                        let local_arc = res_acc.clone();
                        local_arc.lock().unwrap().insert(current_acc.clone());
                    }

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        current_to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                    );
                }
            }
            "join" => {
                let new_str = join::join(strs);
                let mut current_acc = acc.clone();
                let current_to_use = to_use.clone();
                current_acc.push((current.clone(), 1));
                let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                if candidates.len() > 0 {
                    let local_arc = res_acc.clone();
                    local_arc.lock().unwrap().insert(current_acc.clone());
                }

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    current_to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                );
            }
            "permute" => {
                let mut current_permutations = permute::init();
                while let Some(PermuteArgs { permutations: next }) =
                    permute::next(current_permutations)
                {
                    let new_str = permute::decrypt_internal(strs.clone(), next.clone());
                    let mut current_acc = acc.clone();
                    let current_to_use = to_use.clone();
                    current_acc.push((current.clone(), 1));
                    let candidates = candidates::find_candidates(new_str.clone(), clues.clone());

                    if candidates.len() > 0 {
                        let local_arc = res_acc.clone();
                        local_arc.lock().unwrap().insert(current_acc.clone());
                    }
                    current_permutations = PermuteArgs { permutations: next };

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        current_to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                    );
                }
            }
            _ => todo!(),
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

    // #[test]
    // fn it_works_2() {
    //     let decryptors = cryptors::filter_decryptors(vec![]);
    //     for (d, _, decrypt, encrypt) in decryptors.iter() {
    //         assert_eq!(
    //             decrypt(encrypt(vec!["SOME STRING 123 !".to_string()], 1), 1),
    //             vec!["SOME STRING 123 !"],
    //             "error with {}",
    //             &d
    //         )
    //     }
    // }

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
