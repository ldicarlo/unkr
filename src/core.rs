use super::combinator;
use super::cryptors::get_decryptors;
use super::encrypt;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn brute_force_decrypt(str: String) {
    let result = brute_force_strings(str);
    println!("Result: {:?}", result);
}

pub fn internal_brute_force_decrypt(str: String) -> BTreeSet<Vec<(u8, u64)>> {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decryptors = get_decryptors();

    for i in combinator::combinate_strings(decryptors.iter().map(|(id, _, _, _, _)| *id).collect())
    {
        loop_decrypt(results_accumulator.clone(), vec![], i, vec![str.clone()]);
    }

    let result: BTreeSet<Vec<(u8, u64)>> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn brute_force_strings(str: String) -> BTreeSet<Vec<(String, u64)>> {
    internal_brute_force_decrypt(str)
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

            if is_candidate(new_str.clone()) {
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
            );
        }
    }
}

fn is_candidate(strs: Vec<String>) -> bool {
    strs.iter().any(|str| {
        str.contains("CLOCK")
            || str.contains("BERLIN")
            || str.contains("NORTH")
            || str.contains("EAST")
    })
}

fn parse_parameter(parameter: &String) -> (String, u64) {
    let split: Vec<&str> = parameter.split(':').collect();
    (
        (split.get(0).unwrap()).to_string(),
        split
            .get(1)
            .map(|s| s.clone().parse::<u64>().unwrap())
            .unwrap_or(1),
    )
}

pub fn decrypt(strs: Vec<String>, decryptors: Vec<String>) -> Vec<String> {
    let list = get_decryptors();

    decryptors
        .iter()
        .map(parse_parameter)
        .fold(strs, |acc, (decryptor_name, seed)| {
            let (_, _, _, current_decryptor, _) = list
                .iter()
                .into_iter()
                .find(|(_, name, _, _, _)| *name == decryptor_name)
                .unwrap();
            current_decryptor(acc, seed)
        })
}

pub fn print_encrypt(str: String, decryptors: Vec<String>) {
    encrypt::encrypt(vec![str], decryptors)
        .iter()
        .for_each(|s| println!("{}", s));
}

pub fn print_decrypt(str: String, decryptors: Vec<String>) {
    let result = decrypt(vec![str], decryptors).join("");
    println!("{}", result);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            BTreeSet::new(),
            internal_brute_force_decrypt("str".to_string())
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
    fn parse_params() {
        assert_eq!(
            ("caesar".to_string(), 1),
            parse_parameter(&"caesar:1".to_string())
        );
        assert_eq!(
            ("caesar".to_string(), 100),
            parse_parameter(&"caesar:100".to_string())
        );
        assert_eq!(
            ("caesar".to_string(), 1),
            parse_parameter(&"caesar".to_string())
        );
    }

    #[test]
    fn brute_force_1() {
        assert_eq!(
            true,
            // encrypt --string BERLIN -- caesar:10 atbash caesar:5
            brute_force_strings("TQDJMH".to_string()).contains(&vec![
                ("caesar".to_string(), 10),
                ("atbash".to_string(), 0),
                ("caesar".to_string(), 5)
            ])
        );
    }
}
