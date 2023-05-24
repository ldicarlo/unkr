mod atbash;
mod base;
mod brute_force;
mod cache;
mod caesar;
mod candidates;
mod char_utils;
mod colorize;
mod combinator;
mod cryptors;
mod cut;
mod decrypt;
mod dispatcher;
mod encrypt;
mod fuzzer;
mod indexcrypt;
mod join;
mod models;
mod parser;
mod reverse;
mod swap;
mod transpose;
mod vigenere;
use std::io;
mod console;

use clap::{Parser, Subcommand};
mod permute;
fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encrypt { string, steps } => string
            .map(|str| encrypt::print_encrypt(vec![str], steps.clone()))
            .unwrap_or_else(|| {
                encrypt::print_encrypt(
                    io::stdin()
                        .lines()
                        .map(|l| l.unwrap())
                        .collect::<Vec<String>>(),
                    steps,
                )
            }),
        Commands::Decrypt { string, steps } => string
            .map(|str| decrypt::print_decrypt(vec![str], steps.clone()))
            .unwrap_or_else(|| {
                decrypt::print_decrypt(
                    io::stdin()
                        .lines()
                        .map(|l| l.unwrap())
                        .collect::<Vec<String>>(),
                    steps,
                )
            }),
        Commands::BruteForce {
            string,
            clues,
            decryptors,
            steps,
            threads,
        } => brute_force::brute_force_decrypt(
            string,
            clues,
            steps,
            decryptors,
            threads,
            String::from("cache"),
        ),
        Commands::GetDecryptors { decryptors } => println!(
            "{:?}",
            if decryptors.len() == 0 {
                cryptors::get_decryptors()
            } else {
                decryptors
                    .iter()
                    .map(|str| parser::read_bruteforce_parameters(str.to_string()))
                    .collect()
            }
        ),
        Commands::GetCombinations {
            elements_count,
            picks,
        } => combinator::print_combine_elements(elements_count, picks),
        Commands::Fuzz { length, rules } => fuzzer::fuzz_from("".to_string(), length, 27, rules),
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Encrypt {
        /// String to try to encrypt (defaults to stdin)
        #[arg(short, long)]
        string: Option<String>,

        /// Steps list as atbash:1 caesar <DECRYPTOR>[:<SEED>] ...
        #[arg(last = true)]
        steps: Vec<String>,
    },
    Decrypt {
        /// String to try to decrypt (defaults to stdin)
        #[arg(short, long)]
        string: Option<String>,
        /// Steps list as atbash:1 caesar <DECRYPTOR>[:<SEED>] ...
        #[arg(last = true)]
        steps: Vec<String>,
    },
    BruteForce {
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,

        /// Consecutive steps in the bruteforce attempt
        #[arg(long)]
        steps: u8,
        /// filter decryptors to use (empty means all)
        #[arg(long)]
        decryptors: Vec<String>,
        /// words to search for (cannot be empty)
        #[arg(long)]
        clues: Vec<String>,
        /// threads to run
        #[arg(long)]
        threads: u8,
    },
    GetDecryptors {
        /// filter decryptors to use (empty means all)
        #[arg(long)]
        decryptors: Vec<String>,
    },
    GetCombinations {
        /// Consecutive steps in the bruteforce attempt
        #[arg(long)]
        elements_count: u8,
        /// Consecutive steps in the bruteforce attempt
        #[arg(long)]
        picks: u8,
    },
    Fuzz {
        /// max_length of output
        #[arg(short, long)]
        length: usize,

        /// Rules to restrict output
        #[arg(long)]
        rules: Vec<String>,
    },
}
