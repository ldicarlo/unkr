mod atbash;
mod base;
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
mod fuzzer;
mod indexcrypt;
mod join;
mod models;
mod parser;
mod reverse;
mod swap;
mod transpose;
mod vigenere;
use clap::{Parser, Subcommand};
mod permute;
fn main() {
    let args = Cli::parse();

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
        Commands::Fuzz { length } => fuzzer::fuzz_from("".to_string(), length, 27),
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
    Fuzz {
        #[arg(short, long)]
        length: usize,
    },
}
