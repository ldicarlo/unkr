mod atbash;
mod caesar;
mod combinator;
mod core;
mod fold;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::collections::BTreeMap;
fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Encrypt { string , decryptors, seeds} => println!("TODO {} {:?} {:?}", string,decryptors,seeds),
        Commands::Decrypt { string } => todo!(),
        Commands::BruteForce { string } => core::brute_force_decrypt(string),
        Commands::GetDecryptors {} => println!(
            "{:?}",
            core::get_decryptors()
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
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,
        /// Decryptors List
        #[arg(short, long)]
        decryptors: Vec<String>,
        /// seeds list
        #[arg(long)]
        seeds: Vec<u64>,


        
    },
    Decrypt {
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,
    },
    BruteForce {
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,
    },
    GetDecryptors {},
}
