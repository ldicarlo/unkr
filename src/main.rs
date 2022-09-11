mod atbash;
mod caesar;
mod combinator;
mod decryptors;
mod fold;
use decryptors::Decryptor;
fn main() {
    let str = "OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR".to_string().to_uppercase();
    let at_bash = atbash::AtBash {};
    for i in 0..at_bash.get_max_seed() {
        let decrypted = at_bash.decrypt(str.clone(), i);
        if is_candidate(str.clone()) {
            println!("CANDIDATE FOUND: {}", decrypted);
        }
    }
}

fn is_candidate(str: String) -> bool {
    str.contains("CLOCK") || str.contains("BERLIN") || str.contains("NORTH") || str.contains("EAST")
}

fn get_decrypters() -> Vec<impl Decryptor> {
    let v: Vec<dyn Decryptor> = vec![atbash::AtBash {}, caesar::Caesar {}];
    v
}
