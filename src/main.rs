mod atbash;
mod caesar;
mod combinator;
mod core;
mod cryptors;
mod cut;
mod fold;
mod reverse;
mod vigenere;
use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Encrypt { string, steps } => core::print_decrypt(string, steps),
        Commands::Decrypt { string, steps } => core::print_encrypt(string, steps),
        Commands::BruteForce { string } => core::brute_force_decrypt(string),
        Commands::GetDecryptors {} => println!(
            "{:?}",
            cryptors::get_decryptors()
                .iter()
                .map(|(id, str, _, _, _)| (*id, str.clone()))
                .collect::<BTreeMap<u8, std::string::String>>()
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
        // #[arg(short, long)]
        // clues: Vec<String>,
    },
    GetDecryptors {},
}
