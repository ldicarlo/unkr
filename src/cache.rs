use crate::mapper;
use crate::models::{self, DoneLine, PartialLine};
use std::collections::BTreeSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn done_string(
    models::CacheArgs {
        md5_clues,
        md5_string,
        path,
    }: models::CacheArgs,
) -> (String, String) {
    (
        format!("{}/{}/{}/", path, md5_string, md5_clues),
        String::from("done"),
    )
}

fn hits_string(
    models::CacheArgs {
        md5_clues,
        md5_string,
        path,
    }: models::CacheArgs,
) -> (String, String) {
    (
        format!("{}/{}/{}", path, md5_string, md5_clues),
        String::from("hits"),
    )
}

fn partial_string(
    models::CacheArgs {
        md5_clues,
        md5_string,
        path,
    }: models::CacheArgs,
) -> (String, String) {
    (
        format!("{}/{}/{}", path, md5_string, md5_clues),
        String::from("partials"),
    )
}

pub fn get_done_cache(cache_args: models::CacheArgs) -> BTreeSet<models::DoneLine> {
    let (done_folder, done_file) = done_string(cache_args);
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
    cache
}

pub fn get_partial_cache(cache_args: models::CacheArgs) -> BTreeSet<models::PartialLine> {
    let (done_folder, done_file) = partial_string(cache_args);
    fs::create_dir_all(done_folder.clone()).unwrap();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/{}", done_folder, done_file))
        .expect(&format!("Not found: {}/{}", done_folder, done_file));
    let mut cache: BTreeSet<models::PartialLine> = BTreeSet::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(format!("{}/{}", done_folder, done_file))
        .unwrap();

    for result in rdr.records() {
        let record: models::SerializablePartialLine = result
            .expect("Failed to deserialize element.")
            .deserialize(None)
            .expect("Failed to deserialize element.");
        let mut rdr2 = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b':')
            .from_reader(record.cryptor.as_bytes());
        for result2 in rdr2.records() {
            cache.insert(PartialLine {
                cryptor: result2.unwrap().deserialize(None).unwrap(),
                tail: record.tail.clone(),
            });
        }
    }

    cache
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

pub fn push_hit(cache_args: models::CacheArgs, hit_line: models::HitLine) {
    let (hits_folder, hits_file) = hits_string(cache_args);

    push_line(
        hits_folder,
        hits_file,
        mapper::hit_to_string(hit_line.clone()),
    );
}

pub fn push_done(
    done_line: DoneLine,
    models::CacheArgs {
        md5_clues,
        md5_string,
        path,
    }: models::CacheArgs,
) {
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
        format!("{}/{}/{}", path, md5_string, md5_clues),
        String::from("done"),
        result,
    );
}

pub fn push_partial(
    partial_line: PartialLine,
    models::CacheArgs {
        md5_clues,
        md5_string,
        path,
    }: models::CacheArgs,
) {
    let result = mapper::partial_to_string(partial_line);
    push_line(
        format!("{}/{}/{}", path, md5_string, md5_clues),
        String::from("partials"),
        result,
    );
}

pub fn already_done(cache: BTreeSet<DoneLine>, done_line: DoneLine) -> bool {
    cache.contains(&done_line)
}

pub fn partial_done(cache: BTreeSet<PartialLine>, partial_line: PartialLine) -> bool {
    cache.contains(&partial_line)
}

pub fn prepare_cache_args(path: String, str: String, clues: Vec<String>) -> models::CacheArgs {
    models::CacheArgs {
        path,
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

#[cfg(test)]
pub mod tests {
    use std::fs;

    use crate::{
        cache::{prepare_cache_args, push_done, unique_sorted_clues},
        models::{CacheArgs, DoneLine},
    };

    use super::{already_done, get_done_cache, push_line};

    pub fn test_cache_name() -> String {
        String::from("cache-tests")
    }

    pub fn empty_test_cache() {
        fs::remove_dir_all(test_cache_name()).expect("cannot remove dir");
    }

    #[test]
    fn cache_parameters() {
        assert_eq!(
            prepare_cache_args(
                test_cache_name(),
                String::from("STRING"),
                vec![String::from("CLUE1"), String::from("CLUE2")]
            ),
            CacheArgs {
                path: test_cache_name(),
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
        empty_test_cache();
        let CacheArgs {
            path,
            md5_string,
            md5_clues,
        } = prepare_cache_args(
            test_cache_name(),
            String::from("STRING"),
            vec![String::from("CLUE1"), String::from("CLUE2")],
        );
        let full_directory = format!("{}/{}/{}", test_cache_name(), md5_string, md5_clues);
        push_line(
            full_directory.clone(),
            String::from("done"),
            String::from("Vigenere Join Permute;Vigenere:3:3"),
        );
        let cache = get_done_cache(CacheArgs {
            path: path.clone(),
            md5_string: md5_string.clone(),
            md5_clues: md5_clues.clone(),
        });
        let done_line = DoneLine {
            args: Some(String::from("Vigenere:3:3")),
            combinations: String::from("Vigenere Join"),
        };
        assert_eq!(already_done(cache.clone(), done_line.clone()), false);
        push_done(
            done_line.clone(),
            CacheArgs {
                path: path.clone(),
                md5_string: md5_string.clone(),
                md5_clues: md5_clues.clone(),
            },
        );
        let updated_cache = get_done_cache(CacheArgs {
            path: path.clone(),
            md5_string: md5_string.clone(),
            md5_clues: md5_clues.clone(),
        });
        assert_eq!(already_done(updated_cache, done_line.clone()), true);
    }
}
