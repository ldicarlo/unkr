use super::combinator;
use crate::atbash;
use crate::caesar;
use crate::candidates;
use crate::cryptors;
use crate::cut;
use crate::join;
use crate::models;
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
) -> BTreeSet<String> {
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

    let result: BTreeSet<String> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn threaded_function(
    thread_number: u8,
    combinations: Vec<Vec<u8>>,
    results_accumulator: Arc<Mutex<BTreeSet<String>>>,
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
            None,
            vec.clone(),
            vec![str.clone()],
            clues.clone(),
            // cache.clone(),
            decryptors_filtered.clone(),
            None,
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
) -> BTreeSet<String> {
    internal_brute_force_decrypt(str, clues, steps, decryptors_filtered.clone(), threads)
    // .iter()
    // .map(|vec| {
    //     vec.iter()
    //         .map(|(current_id, seed)| {
    //             let d_name = cryptors::filter_decryptors(decryptors_filtered.clone())
    //                 .into_iter()
    //                 .nth((*current_id).into())
    //                 .unwrap();
    //             (d_name, *seed)
    //         })
    //         .collect()
    // })
    // .collect()
}

fn loop_decrypt(
    res_acc: Arc<Mutex<BTreeSet<String>>>,
    acc: Option<String>,
    mut to_use: Vec<u8>,
    strs: Vec<String>,
    clues: Vec<String>,
    // mut cache: BTreeSet<(Vec<String>, Vec<u8>, u64)>,
    decryptors_filtered: Vec<String>,
    previous: Option<String>,
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
                if previous
                    .map(|prev| atbash::skip_if_previous_in().contains(&prev))
                    .unwrap_or(false)
                {
                    return;
                }
                let new_str: Vec<String> = atbash::decrypt(strs.clone());

                let current_acc = process_new_str(
                    res_acc.clone(),
                    acc,
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                );

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    Some(cryptor_name.clone()),
                );
            }
            "caesar" => {
                for s in 0..26 {
                    let new_str = caesar::decrypt(strs.clone(), models::NumberArgs { number: s });
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        Some(cryptor_name.clone()),
                    );
                }
            }
            "reverse" => {
                if previous
                    .map(|prev: String| reverse::skip_if_previous_in().contains(&prev))
                    .unwrap_or(false)
                {
                    return;
                }
                let new_str = reverse::decrypt(strs.clone());
                let current_acc = process_new_str(
                    res_acc.clone(),
                    acc,
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                );

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    Some(cryptor_name),
                );
            }
            "transpose" => {
                for s in 1..strs.first().map(|s| s.len()).unwrap_or(0) {
                    let new_str =
                        transpose::decrypt(strs.clone(), models::NumberArgs { number: s as u64 });
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        Some(cryptor_name.clone()),
                    );
                }
            }
            "vigenere" => {
                for _ in 0..vigenere::get_max_seed() {
                    let new_str = vigenere::decrypt(strs.clone(), vigenere::init());
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}:{}", "TO", "FIX"),
                        new_str.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        Some(cryptor_name.clone()),
                    );
                }
            }
            "cut" => {
                for s in 0..strs.first().map(|s| s.len()).unwrap_or(0) {
                    let new_str =
                        cut::encrypt(strs.clone(), models::NumberArgs { number: s as u64 });
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{}", s),
                        new_str.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        Some(cryptor_name.clone()),
                    );
                }
            }
            "join" => {
                if previous
                    .map(|prev| join::skip_if_previous_in().contains(&prev))
                    .unwrap_or(false)
                {
                    return;
                }
                let new_str = join::decrypt(strs.clone());
                let current_acc = process_new_str(
                    res_acc.clone(),
                    acc.clone(),
                    clues.clone(),
                    cryptor_name.clone(),
                    new_str.clone(),
                );

                loop_decrypt(
                    res_acc.clone(),
                    current_acc,
                    to_use.clone(),
                    new_str.clone(),
                    clues.clone(),
                    decryptors_filtered.clone(),
                    Some(cryptor_name.clone()),
                );
            }
            "permute" => {
                let mut current_permutations = permute::init();
                while let Some(next) = permute::next(current_permutations.clone()) {
                    let new_str = permute::decrypt(strs.clone(), next.clone());
                    let current_acc = process_new_str(
                        res_acc.clone(),
                        acc.clone(),
                        clues.clone(),
                        cryptor_name.clone() + &format!(":{:?}", current_permutations),
                        new_str.clone(),
                    );

                    loop_decrypt(
                        res_acc.clone(),
                        current_acc,
                        to_use.clone(),
                        new_str.clone(),
                        clues.clone(),
                        decryptors_filtered.clone(),
                        Some(cryptor_name.clone()),
                    );
                    current_permutations = next;
                }
            }
            _ => todo!(),
        }
    }
}

fn process_new_str(
    res_acc: Arc<Mutex<BTreeSet<String>>>,
    acc: Option<String>,
    clues: Vec<String>,
    // mut cache: BTreeSet<(Vec<String>, Vec<u8>, u64)>,
    cryptor_str: String,
    new_str: Vec<String>,
) -> Option<String> {
    let current_acc = acc
        .map(|existing| existing + " " + &cryptor_str.clone())
        .unwrap_or(cryptor_str.clone());
    let candidates =
        candidates::find_and_print_candidates(new_str.clone(), clues.clone(), current_acc.clone());

    if candidates.len() > 0 {
        let local_arc = res_acc.clone();
        local_arc.lock().unwrap().insert(current_acc.clone());
    }
    Some(current_acc)
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
            ),
            vec![
                "caesar:5 atbash".to_string(),
                "atbash caesar:21".to_string(),
            ]
            .into_iter()
            .collect::<BTreeSet<String>>(),
        );
    }
}
