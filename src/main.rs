mod atbash;
mod caesar;
mod candidates;
mod char_utils;
mod colorize;
mod combinator;
mod core;
mod cryptors;
mod cut;
mod decrypt;
mod encrypt;
mod indexcrypt;
mod join;
mod models;
mod parser;
mod reverse;
mod swap;
mod transpose;
mod vigenere;
use clap::{Parser, Subcommand};
use std::{collections::BTreeSet, time::Instant};
fn main() {
    let args = Cli::parse();
    let start = Instant::now();

    match args.command {
        Commands::Encrypt { string, steps } => encrypt::print_encrypt(string, steps),
        Commands::Decrypt { string, steps } => decrypt::print_decrypt(string, steps),
        Commands::BruteForce {
            string,
            clues,
            decryptors,
            steps,
            threads,
        } => core::brute_force_decrypt(string, clues, steps, decryptors, threads),
        Commands::GetDecryptors { decryptors } => println!(
            "{:?}",
            cryptors::filter_decryptors(decryptors)
                .iter()
                .map(|(str, _, _, _)| (str.clone()))
                .collect::<BTreeSet<std::string::String>>()
        ),
        Commands::GetCombinations {
            elements_count,
            picks,
        } => combinator::print_combine_elements(elements_count, picks),
    };
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
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
        /// String to try to encrypt
        #[arg(short, long)]
        string: String,
        /// Steps list as atbash:1 caesar <DECRYPTOR>[:<SEED>] ...
        #[arg(last = true)]
        steps: Vec<String>,
    },
    Decrypt {
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,
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
}
