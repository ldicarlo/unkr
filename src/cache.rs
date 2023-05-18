use crate::models;
use std::collections::BTreeSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

// get Arc
// trigger write

pub fn get_hits_cache(directory: String) -> Arc<Mutex<bool>> {
    Arc::new(Mutex::new(true))
}
pub fn get_done_cache(directory: String) -> Arc<Mutex<BTreeSet<models::DoneLine>>> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .open(format!("{}/{}", directory, "done"))
        .unwrap();
    let mut result: BTreeSet<models::DoneLine> = BTreeSet::new();

    // Arc::new(Mutex::new(result))

    // let mut rdr = csv::Reader::from_reader("assets/quotes.csv").expect("assets/quotes.csv not readable.");
    // for result in rdr.records() {
    //     let record: models::YahooQuoteRecord = result
    //         .expect("Failed to deserialize element.")
    //         .deserialize(None)
    //         .expect("Failed to deserialize element.");
    //     quotes.push(record)
    // }
    // quotes
}

pub fn push_line(directory: String, file_name: String, line: String) {
    fs::create_dir_all(directory).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();
    writeln!(file, "{}", line).unwrap();
}

pub fn push_hit(directory: String, cache: Arc<Mutex<bool>>, hit_line: models::HitLine) {
    if let Ok(c) = cache.try_lock() {
        push_line(
            directory,
            String::from("hits"),
            hit_to_string(hit_line.clone()),
        );
    }
}

pub fn push_done() {}

pub fn exists() {}

pub fn prepare_cache_args(str: String, clues: Vec<String>) -> models::CacheArgs {
    models::CacheArgs {
        md5_string: hash(str),
        md5_clues: hash(unique_sorted_clues(clues)),
    }
}

fn unique_sorted_clues(clues: Vec<String>) -> String {
    clues
        .into_iter()
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<String>>()
        .join(" ")
}

fn hash(str: String) -> String {
    format!("{:x}", md5::compute(str.into_bytes()))
}

fn hit_to_string(hit_line: models::HitLine) -> String {
    format!("{};{}", hit_line.result, hit_line.args)
}

#[cfg(test)]
mod tests {
    use crate::{
        cache::{prepare_cache_args, unique_sorted_clues},
        models,
    };

    use super::push_line;

    #[test]
    fn cache_parameters() {
        assert_eq!(
            prepare_cache_args(
                String::from("STRING"),
                vec![String::from("CLUE1"), String::from("CLUE2")]
            ),
            models::CacheArgs {
                md5_string: String::from("63b588d5559f64f89a416e656880b949"),
                md5_clues: String::from("27a711f10fa00512ba38ad3608352b37")
            }
        )
    }

    #[test]
    fn cache_dedup_parameters() {
        assert_eq!(
            unique_sorted_clues(vec![String::from("CLUE1"), String::from("CLUE2")]),
            unique_sorted_clues(vec![
                String::from("CLUE2"),
                String::from("CLUE1"),
                String::from("CLUE2"),
                String::from("CLUE2"),
                String::from("CLUE1"),
                String::from("CLUE2"),
            ]),
        )
    }

    #[test]
    fn can_push_line() {
        push_line(
            String::from("cache-tests"),
            String::from("done"),
            String::from("hello"),
        );
    }
}
