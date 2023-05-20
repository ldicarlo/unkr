use crate::brute_force;
use crate::models::{self, DoneLine};
use std::collections::BTreeSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

fn done_string(
    cache_directory: String,
    models::CacheArgs {
        md5_clues,
        md5_string,
    }: models::CacheArgs,
) -> (String, String) {
    (
        format!("{}/{}/{}/", cache_directory, md5_string, md5_clues),
        String::from("done"),
    )
}

fn hits_string(
    cache_directory: String,
    models::CacheArgs {
        md5_clues,
        md5_string,
    }: models::CacheArgs,
) -> (String, String) {
    (
        format!("{}/{}/{}", cache_directory, md5_string, md5_clues),
        String::from("hits"),
    )
}

pub fn get_hits_cache(directory: String) -> Arc<Mutex<bool>> {
    Arc::new(Mutex::new(true))
}
pub fn get_done_cache(
    cache_directory: String,
    cache_args: models::CacheArgs,
) -> Arc<Mutex<BTreeSet<models::DoneLine>>> {
    let (done_folder, done_file) = done_string(cache_directory, cache_args);
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/{}", done_folder, done_file))
        .unwrap();
    let mut cache: BTreeSet<models::DoneLine> = BTreeSet::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(format!("{}/{}", done_folder, done_file))
        .unwrap();

    for result in rdr.records() {
        println!("{:?}", result);
        let record: models::DoneLine = result
            .expect("Failed to deserialize element.")
            .deserialize(None)
            .expect("Failed to deserialize element.");
        cache.insert(record);
    }
    Arc::new(Mutex::new(cache))
}

pub fn push_line(full_directory: String, file_name: String, line: String) {
    fs::create_dir_all(full_directory.clone()).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}/{}", full_directory, file_name))
        .unwrap();
    writeln!(file, "{}", line).unwrap();
}

pub fn push_hit(
    directory: String,
    cache_args: models::CacheArgs,
    cache: Arc<Mutex<bool>>,
    hit_line: models::HitLine,
) {
    let (hits_folder, hits_file) = hits_string(directory, cache_args);

    if let Ok(c) = cache.try_lock() {
        push_line(
            format!("{}/{}", hits_folder, hits_file),
            String::from("hits"),
            hit_to_string(hit_line.clone()),
        );
    }
}

pub fn push_done(
    directory: String,
    cache: Arc<Mutex<BTreeSet<models::DoneLine>>>,
    done_line: DoneLine,
    models::CacheArgs {
        md5_clues,
        md5_string,
    }: models::CacheArgs,
) {
    if let Ok(mut c) = cache.try_lock() {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_writer(vec![]);
        writer.serialize(done_line.clone()).expect("FAIL");
        let result = String::from_utf8(writer.into_inner().expect("Cannot convert utf8"))
            .expect("Cannot convert utf8");
        push_line(directory, String::from("done"), result);
        c.insert(done_line);
    }
}

pub fn already_done(cache: Arc<Mutex<BTreeSet<models::DoneLine>>>, done_line: DoneLine) -> bool {
    if let Ok(c) = cache.try_lock() {
        c.contains(&done_line)
    } else {
        false
    }
}

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
pub fn to_done(
    brute_force_args: Vec<models::BruteForceCryptor>,
    combinations: String,
) -> models::DoneLine {
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        cache::{prepare_cache_args, push_done, unique_sorted_clues},
        models,
    };

    use super::{already_done, get_done_cache, push_line};

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
    fn done_workflow() {
        push_line(
            String::from("cache-tests"),
            String::from("done"),
            String::from("vigenere join permute;vigenere:3:3"),
        );
        let cache = get_done_cache(
            String::from("cache-tests"),
            prepare_cache_args(
                String::from("STRING"),
                vec![String::from("CLUE1"), String::from("CLUE2")],
            ),
        );
        let done_line = models::DoneLine {
            args: Some(String::from("vigenere:3:3")),
            combinations: String::from("vigenere join"),
        };
        assert_eq!(already_done(cache.clone(), done_line.clone()), false);
        push_done(
            String::from("cache-tests"),
            cache.clone(),
            done_line.clone(),
            models::CacheArgs {
                md5_string: String::from("abc"),
                md5_clues: String::from("123"),
            },
        );
        assert_eq!(already_done(cache.clone(), done_line.clone()), true);
        fs::remove_file("cache-tests/abc/123/done").expect("cannot delete file");
    }
}
