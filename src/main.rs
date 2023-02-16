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
use std::collections::BTreeSet;
fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Encrypt { string, steps } => encrypt::print_encrypt(string, steps),
        Commands::Decrypt { string, steps } => decrypt::print_decrypt(string, steps),
        Commands::BruteForce {
            string,
            clues,
            steps,
        } => core::brute_force_decrypt(string, clues, steps),
        Commands::GetDecryptors {} => println!(
            "{:?}",
            cryptors::get_decryptors()
                .iter()
                .map(|(str, _, _, _)| (str.clone()))
                .collect::<BTreeSet<std::string::String>>()
        ),
    }
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
        #[arg(last = true)]
        clues: Vec<String>,
    },
    GetDecryptors {},
}
