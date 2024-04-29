use crate::cache;
use crate::console;
use crate::console::PrintableMessage;
use crate::models::CacheArgs;
use crate::models::Cryptor;
use crate::models::HitLine;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn candidate_receiver(
    r: Receiver<(Vec<String>, Vec<String>, Vec<Cryptor>)>,
    cache_args: CacheArgs,
    result_accumulator: Arc<Mutex<BTreeSet<String>>>,
    console_sender: Sender<PrintableMessage>,
    pretty: bool,
) {
    r.iter().for_each(|(a, b, c)| {
        let result = find_and_print_candidates(a, b, c.clone(), console_sender.clone(), pretty);

        if result.len() > 0 {
            result_accumulator
                .lock()
                .unwrap()
                .insert(format!("{:?}", c.clone()));
            cache::push_hit(
                cache_args.clone(),
                HitLine {
                    args: c.clone(),
                    result: result.clone().join(""),
                },
            )
        }
    });
}

pub fn find_and_print_candidates(
    strs: Vec<String>,
    clues: Vec<String>,
    cryptor_str: Vec<Cryptor>,
    console_sender: Sender<PrintableMessage>,
    pretty: bool,
) -> Vec<String> {
    let candidates = find_candidates(strs.clone(), clues.clone());
    if candidates.len() > 0 && pretty {
        console_sender
            .send(console::PrintableMessage::Default(format!(
                "{:?} {:?}",
                candidates, cryptor_str
            )))
            .unwrap();
        println!("{:?} {}", candidates, cryptor_str);
    }
    candidates
}
pub fn find_candidates(strs: Vec<String>, clues: Vec<String>) -> Vec<String> {
    check_string_for_candidates(&strs.join(""), &clues)
}

fn check_string_for_candidates(string: &String, clues: &Vec<String>) -> Vec<String> {
    let step1 = clue_is_in_string(string, clues);
    if step1.len() > 0 {
        return step1;
    }
    step1
}

fn clue_is_in_string(string: &String, clues: &Vec<String>) -> Vec<String> {
    clues
        .iter()
        .filter(|clue| string.contains(*clue))
        .map(|clue| format!("{} was found in {}", clue, string))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            clue_is_in_string(&String::from("STRING"), &vec![String::from("IN")]),
            vec!["IN was found in STRING"]
        )
    }
}
