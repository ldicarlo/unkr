use crate::models;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::hash;
use std::hash::Hash;
use std::hash::Hasher;
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

#[cfg(test)]
mod tests {
    use crate::{
        cache::{prepare_cache_args, unique_sorted_clues},
        models,
    };

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
}
