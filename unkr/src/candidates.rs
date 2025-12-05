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
) {
    r.iter().for_each(|(a, b, c)| {
        let result = find_and_print_candidates(a, b, c.clone(), console_sender.clone());

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
) -> Vec<String> {
    let candidates = find_candidates(strs.clone(), clues.clone());
    if candidates.len() > 0 {
        console_sender
            .send(console::PrintableMessage::Default(format!(
                "{:?} {:?}",
                candidates, cryptor_str
            )))
            .unwrap();
    }
    candidates
}
pub fn find_candidates(strs: Vec<String>, clues: Vec<String>) -> Vec<String> {
    clue_is_in_string(&strs.join(""), &clues)
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
