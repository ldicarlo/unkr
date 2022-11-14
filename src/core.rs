use super::atbash;
use super::caesar;
use super::combinator;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn brute_force_decrypt(str: String) {
    let result = internal_brute_force_decrypt(str);
    println!("Result: {:?}", result);
}

pub fn internal_brute_force_decrypt(str: String) -> BTreeSet<Vec<(u8, u64)>> {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    let decryptors = get_decryptors();

    for i in combinator::combinate_strings(decryptors.iter().map(|(id, _, _, _, _)| *id).collect())
    {
        println!("brute_force_decrypt {:?}", i);
        loop_decrypt(results_accumulator.clone(), vec![], i, str.clone());
    }

    let result: BTreeSet<Vec<(u8, u64)>> = results_accumulator.lock().unwrap().to_owned();
    result
}

fn loop_decrypt(
    res_acc: Arc<Mutex<BTreeSet<Vec<(u8, u64)>>>>,
    acc: Vec<(u8, u64)>,
    mut to_use: Vec<u8>,
    str: String,
) {
    let local_arc = res_acc.clone();
    if let Some(current) = to_use.pop() {
        let (_, _, seed, decrypt, _) = get_decryptors()
            .into_iter()
            .find(|(id, _, _, _, _)| *id == current)
            .unwrap();
        for s in 1..seed() {
            let new_str = decrypt(str.clone(), s);
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

fn is_candidate(str: String) -> bool {
    str.contains("CLOCK") || str.contains("BERLIN") || str.contains("NORTH") || str.contains("EAST")
}

pub fn get_decryptors() -> Vec<(
    u8,
    String,
    Box<dyn Fn() -> u64>,
    Box<dyn Fn(String, u64) -> String>,
    Box<dyn Fn(String, u64) -> String>,
)> {
    vec![
        (
            1,
            "atbash".to_string(),
            Box::new(atbash::get_max_seed),
            Box::new(atbash::decrypt),
            Box::new(atbash::decrypt),
        ),
        (
            2,
            "caesar".to_string(),
            Box::new(caesar::get_max_seed),
            Box::new(caesar::decrypt),
            Box::new(caesar::encrypt),
        ),
    ]
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

pub fn decrypt(str: String, decryptors: Vec<String>) -> String {
    let list = get_decryptors();

    decryptors
        .iter()
        .map(parse_parameter)
        .fold(str, |acc, (decryptor_name, seed)| {
            let (_, _, _, current_decryptor, _) = list
                .iter()
                .into_iter()
                .find(|(_, name, _, _, _)| *name == decryptor_name)
                .unwrap();
            current_decryptor(acc, seed)
        })
}

pub fn encrypt(str: String, decryptors: Vec<String>) -> String {
    let list = get_decryptors();

    decryptors
        .iter()
        .map(parse_parameter)
        .fold(str, |acc, (decryptor_name, seed)| {
            let (_, _, _, _, current_encryptor) = list
                .iter()
                .into_iter()
                .find(|(_, name, _, _, _)| *name == decryptor_name)
                .unwrap();
            current_encryptor(acc, seed)
        })
}

pub fn print_encrypt(str: String, decryptors: Vec<String>) {
    let result = encrypt(str, decryptors);
    println!("{}", result);
}

pub fn print_decrypt(str: String, decryptors: Vec<String>) {
    let result = decrypt(str, decryptors);
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
                decrypt(encrypt("SOME STRING 123 !".to_string(), 1), 1),
                "SOME STRING 123 !",
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
            BTreeSet::new(), // !! target/debug/kryptos encrypt --string BERLIN -- caesar:10 atbash caesar:5
            internal_brute_force_decrypt("TQDJMH".to_string())
        );
    }
}
