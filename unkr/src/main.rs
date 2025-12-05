use clap::{Parser, Subcommand};
use core::cmp::max;
use std::io;
use unkr;

fn main() {
    const SHADER: &[u8] = include_bytes!(env!("gpu.spv"));

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
            pretty,
            runner_thread_numbers,
            runners_threads_total_count,
        } => {
            let threads = runner_thread_numbers.unwrap_or(vec![0]);
            let threads_count = runners_threads_total_count.unwrap_or(max(
                *threads.iter().max().unwrap_or(&0),
                threads.len() as u8,
            ));

            unkr::brute_force_decrypt(
                string,
                clues,
                steps,
                decryptors,
                threads,
                threads_count,
                pretty,
                String::from("cache"),
            )
        }
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
            decryptors,
            pretty,
            intermediate_steps,
            runner_thread_numbers,
            runners_threads_total_count,
        } => {
            let threads = runner_thread_numbers.unwrap_or(vec![0]);
            let threads_count = runners_threads_total_count.unwrap_or(max(
                *threads.iter().max().unwrap_or(&0) + 1,
                threads.len() as u8,
            ));
            unkr::brute_force_unique_combination(
                string,
                clues,
                decryptors,
                threads,
                threads_count,
                String::from("cache"),
                pretty,
                intermediate_steps,
            )
        }
        Commands::RunGpu {} => todo!(),
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
    /// run simple encryption
    #[command(arg_required_else_help = true)]
    Encrypt {
        /// String to try to encrypt (defaults to stdin)
        #[arg(short, long)]
        string: Option<String>,

        /// Steps list as atbash:1 caesar <DECRYPTOR>[:<SEED>] ...
        #[arg(last = true)]
        steps: Vec<String>,
    },
    /// run simple decryption
    Decrypt {
        /// String to try to decrypt (defaults to stdin)
        #[arg(short, long)]
        string: Option<String>,
        /// Steps list as atbash:1 caesar <DECRYPTOR>[:<SEED>] ...
        #[arg(last = true)]
        steps: Vec<String>,
    },
    /// bruteforce from known clues in the text
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

        /// using pretty prints "nicely" (hey it's a shell don't be too picky) the logs
        #[arg(long)]
        pretty: bool,

        /// run as one of multiple runners, or single runner (runner_thread_numbers starts at 0), give this runner a number. It will skip other steps
        /// So for example if you have 2 runners and 2 threads in each that's [0 ,1].
        #[arg(long, value_delimiter = ',', num_args=1..)]
        runner_thread_numbers: Option<Vec<u8>>,

        /// run as one of multiple runners, give the total number of runners threads.
        /// So for example if you have 2 runners and 2 threads in each that's 4.
        /// if not provided, total_count will be the size of `runner_thread_numbers`
        #[arg(long)]
        runners_threads_total_count: Option<u8>,
    },
    /// bruteforce a single combination from known clues in the text
    BruteForceCombination {
        /// String to try to decrypt
        #[arg(short, long)]
        string: String,

        /// words to search for (cannot be empty)
        #[arg(long)]
        clues: Vec<String>,

        /// Combination of BrufteForce params to use
        #[arg(last = true)]
        decryptors: Vec<String>,

        /// using pretty prints "nicely" (hey it's a shell don't be too picky) the logs
        #[arg(long)]
        pretty: bool,

        /// check for clues during the intermediate steps of an encryption
        #[arg(long)]
        intermediate_steps: bool,

        /// run as one of multiple runners, or single runner (runner_thread_numbers starts at 0), give this runner a number. It will skip other steps
        /// So for example if you have 2 runners and 2 threads in each that's [0 ,1].
        #[arg(long, value_delimiter = ',', num_args=1..)]
        runner_thread_numbers: Option<Vec<u8>>,

        /// run as one of multiple runners, give the total number of runners threads.
        /// So for example if you have 2 runners and 2 threads in each that's 4.
        /// if not provided, total_count will be the size of `runner_thread_numbers`
        #[arg(long)]
        runners_threads_total_count: Option<u8>,
    },

    /// list all decryptors
    GetDecryptors {
        /// filter decryptors to use (empty means all)
        #[arg(long)]
        decryptors: Vec<String>,
    },

    /// generate a combination list
    GetCombinations {
        /// Consecutive steps in the bruteforce attempt
        #[arg(long)]
        elements_count: u8,
        /// Consecutive steps in the bruteforce attempt
        #[arg(long)]
        picks: u8,
    },

    /// generate strings knowing length (length 2 -> A -> ZZ)
    Fuzz {
        /// max_length of output
        #[arg(short, long)]
        length: usize,

        /// Rules to restrict output
        /// SortedLettersByPair,EvenCount,UniqueLetters
        #[arg(short, long)]
        rules: Vec<String>,
    },

    RunGpu {},
    //Crossterm {},
}
