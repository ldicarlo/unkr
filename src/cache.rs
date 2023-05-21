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

pub fn get_done_cache(
    cache_directory: String,
    cache_args: models::CacheArgs,
) -> Arc<Mutex<BTreeSet<models::DoneLine>>> {
    let (done_folder, done_file) = done_string(cache_directory, cache_args);
    fs::create_dir_all(done_folder.clone()).unwrap();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/{}", done_folder, done_file))
        .expect(&format!("Not found: {}/{}", done_folder, done_file));
    let mut cache: BTreeSet<models::DoneLine> = BTreeSet::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(format!("{}/{}", done_folder, done_file))
        .unwrap();

    for result in rdr.records() {
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

pub fn push_hit(directory: String, cache_args: models::CacheArgs, hit_line: models::HitLine) {
    let (hits_folder, hits_file) = hits_string(directory, cache_args);

    push_line(hits_folder, hits_file, hit_to_string(hit_line.clone()));
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
            .expect("Cannot convert utf8")
            .trim()
            .to_string();
        push_line(
            format!("{}/{}/{}", directory, md5_string, md5_clues),
            String::from("done"),
            result,
        );
        c.insert(done_line);
    }
}

pub fn already_done(cache: Arc<Mutex<BTreeSet<models::DoneLine>>>, done_line: DoneLine) -> bool {
    if let Ok(c) = cache.try_lock() {
        println!("{:?}", c);
        println!("{:?}", done_line);
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
    brute_force_cryptors: Vec<models::BruteForceCryptor>,
    combinations: Vec<u8>,
) -> models::DoneLine {
    let (left, right) = combinations_string(combination(brute_force_cryptors, combinations));
    models::DoneLine {
        combinations: left,
        args: right,
    }
}

pub fn combinations_string(
    brute_force_cryptors: Vec<models::BruteForceCryptor>,
) -> (String, Option<String>) {
    let strings: Vec<(String, Option<String>)> = brute_force_cryptors
        .iter()
        .map(|c| match c {
            models::BruteForceCryptor::Vigenere(models::BruteForceVigenereArgs {
                alphabet_depth,
                key_depth,
            }) => (
                String::from("Vigenere"),
                Some(format!("Vigenere:{}:{}", alphabet_depth, key_depth)),
            ),
            models::BruteForceCryptor::Cut => (String::from("Cut"), None),
            models::BruteForceCryptor::Caesar => (String::from("Caesar"), None),
            models::BruteForceCryptor::Transpose => (String::from("Transpose"), None),
            models::BruteForceCryptor::AtBash => (String::from("AtBash"), None),
            models::BruteForceCryptor::Reverse => (String::from("Reverse"), None),
            models::BruteForceCryptor::Swap => (String::from("Swap"), None),
            models::BruteForceCryptor::Join => (String::from("Join"), None),
            models::BruteForceCryptor::IndexCrypt => (String::from("IndexCrypt"), None),
            models::BruteForceCryptor::Permute(models::BruteForcePermuteArgs {
                max_permutations,
            }) => (
                String::from("Permute"),
                Some(format!("Permute:{}", max_permutations)),
            ),
        })
        .collect();
    //strings.sort_by_key(|(a, _)| a.clone());
    let left = strings
        .clone()
        .into_iter()
        .map(|(a, _)| a)
        .collect::<Vec<String>>()
        .join(" ");

    let rights = strings
        .into_iter()
        .flat_map(|(_, b)| b)
        .collect::<Vec<String>>();
    let right = if rights.is_empty() {
        None
    } else {
        Some(rights.join(" "))
    };

    (left, right)
}

pub fn combination(
    brute_force_cryptors: Vec<models::BruteForceCryptor>,
    combinations: Vec<u8>,
) -> Vec<models::BruteForceCryptor> {
    let mut result = Vec::new();
    for n in combinations.into_iter() {
        result.push(
            brute_force_cryptors
                .clone()
                .into_iter()
                .nth(n.into())
                .expect("Did not find cryptor index"),
        );
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        cache::{prepare_cache_args, push_done, unique_sorted_clues},
        models,
    };

    use super::{already_done, get_done_cache, push_line, to_done};

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
        let models::CacheArgs {
            md5_string,
            md5_clues,
        } = prepare_cache_args(
            String::from("STRING"),
            vec![String::from("CLUE1"), String::from("CLUE2")],
        );
        let full_directory = format!("cache-tests/{}/{}", md5_string, md5_clues);
        push_line(
            full_directory.clone(),
            String::from("done"),
            String::from("Vigenere Join Permute;Vigenere:3:3"),
        );
        let cache = get_done_cache(
            String::from("cache-tests"),
            models::CacheArgs {
                md5_string: md5_string.clone(),
                md5_clues: md5_clues.clone(),
            },
        );
        let done_line = models::DoneLine {
            args: Some(String::from("Vigenere:3:3")),
            combinations: String::from("Vigenere Join"),
        };
        assert_eq!(already_done(cache.clone(), done_line.clone()), false);
        push_done(
            String::from("cache-tests"),
            cache.clone(),
            done_line.clone(),
            models::CacheArgs {
                md5_string: md5_string.clone(),
                md5_clues: md5_clues.clone(),
            },
        );
        assert_eq!(already_done(cache.clone(), done_line.clone()), true);
        fs::remove_file(format!("{}/done", full_directory)).expect("cannot delete file");
    }

    #[test]
    fn to_done_works() {
        assert_eq!(
            to_done(
                vec![
                    models::BruteForceCryptor::Vigenere(models::BruteForceVigenereArgs {
                        alphabet_depth: 4,
                        key_depth: 7
                    },),
                    models::BruteForceCryptor::Transpose,
                    models::BruteForceCryptor::Caesar,
                ],
                vec![0, 1, 2]
            ),
            models::DoneLine {
                combinations: String::from("Vigenere Transpose Caesar"),
                args: Some(String::from("Vigenere:4:7"))
            }
        )
    }

    #[test]
    fn to_done_no_args_works() {
        assert_eq!(
            to_done(
                vec![
                    models::BruteForceCryptor::Transpose,
                    models::BruteForceCryptor::Caesar,
                ],
                vec![0, 1]
            ),
            models::DoneLine {
                combinations: String::from("Transpose Caesar"),
                args: None
            }
        )
    }
}
