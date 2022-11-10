use super::atbash;
use super::caesar;
use super::combinator;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn brute_force_decrypt(str: String) {
    let results_accumulator = Arc::new(Mutex::new(BTreeSet::new()));
    internal_brute_force_decrypt(results_accumulator.clone(), str);
    println!("Result: {:?}", results_accumulator.lock().unwrap());
}

pub fn internal_brute_force_decrypt(
    results_accumulator: Arc<Mutex<BTreeSet<Vec<(u8, u64)>>>>,
    str: String,
) {
    let decryptors = get_decryptors();

    for i in combinator::combinate_strings(decryptors.iter().map(|(id, _, _, _, _)| *id).collect())
    {
        println!("brute_force_decrypt {:?}", i);
        loop_decrypt(results_accumulator.clone(), vec![], i, str.clone());
    }
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
            // println!(
            //     "loop_decrypt acc:{:?} list:{:?} {} - {} {} -> {}",
            //     current_acc.clone(),
            //     to_use.clone(),
            //     str,
            //     current,
            //     decryptor_id,
            //     new_str.clone()
            // );
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
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let arc = Arc::new(Mutex::new(BTreeSet::new()));
        internal_brute_force_decrypt(arc.clone(), "str".to_string());
        if let Ok(result) = arc.clone().lock() {
            assert_eq!(result.clone(), BTreeSet::new());
        }
    }

    #[test]
    fn it_works_2() {
        let decryptors = get_decryptors();
        for (_,d,_,decrypt,encrypt) in decryptors.iter() {
            assert_eq!(decrypt(encrypt("SOME STRING 123 !".to_string(),1),1), "SOME STRING 123 !", "error with {}", &d) 
        }
    }
}
