use unkr;

use clap::{Parser, Subcommand};
use std::io;
fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encrypt { string, steps } => string
            .map(|str| unkr::print_encrypt(vec![str], steps.clone()))
            .unwrap_or_else(|| {
                unkr::print_encrypt(
                    io::stdin()
                        .lines()
                        .map(|l| l.unwrap())
                        .collect::<Vec<String>>(),
                    steps,
                )
            }),
        Commands::Decrypt { string, steps } => string
            .map(|str| unkr::print_decrypt(vec![str], steps.clone()))
            .unwrap_or_else(|| {
                unkr::print_decrypt(
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
            pretty,
        } => unkr::brute_force_decrypt(
            string,
            clues,
            steps,
            decryptors,
            threads,
            pretty,
            String::from("cache"),
        ),
        Commands::GetDecryptors { decryptors } => println!(
            "{:?}",
            if decryptors.len() == 0 {
                unkr::get_decryptors()
            } else {
                decryptors
                    .iter()
                    .map(|str| unkr::read_bruteforce_parameters(str.to_string()))
                    .collect()
            }
        ),
        Commands::GetCombinations {
            elements_count,
            picks,
        } => unkr::print_combine_elements(elements_count, picks),
        Commands::Fuzz { length, rules } => unkr::fuzz_from("".to_string(), length, 27, rules),
        Commands::BruteForceCombination {
            string,
            clues,
            threads,
            decryptors,
            pretty,
            intermediate_steps,
        } => unkr::brute_force_unique_combination(
            string,
            clues,
            decryptors,
            threads,
            String::from("cache"),
            pretty,
            intermediate_steps,
        ),
        //Commands::Crossterm {} => console::consume_message(),
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
        /// using pretty prints "nicely" (hey it's a shell don't be too picky) the logs
        #[arg(long)]
        pretty: bool,
    },
    BruteForceCombination {
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,

        /// words to search for (cannot be empty)
        #[arg(long)]
        clues: Vec<String>,
        /// threads to run
        #[arg(long)]
        threads: u8,
        /// Combination of BrufteForce params to use
        #[arg(last = true)]
        decryptors: Vec<String>,
        /// using pretty prints "nicely" (hey it's a shell don't be too picky) the logs
        #[arg(long)]
        pretty: bool,
        /// check for clues during the intermediate steps of an encryption
        #[arg(long)]
        intermediate_steps: bool,
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
    //Crossterm {},
}
