mod atbash;
mod caesar;
mod combinator;
mod core;
mod fold;
use clap::Parser;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;
fn main() {
    let args = Args::parse();
    let str = args.string.unwrap_or( "OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR".to_string().to_uppercase());
    let result = Arc::new(Mutex::new(BTreeSet::new()));
    println!("Input string: {}", str.clone());
    core::brute_force_decrypt(result.clone(), str);
    println!("Result: {:?}", result.lock().unwrap());
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to try to decrypt
    #[arg(short, long)]
    string: Option<String>,
}
