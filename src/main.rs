mod atbash;
mod caesar;
mod combinator;
mod decryptors;
mod fold;
use decryptors::Decryptor;
fn main() {
    let str = "OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR".to_string().to_uppercase();
    let decryptors = get_decryptors();

    for i in combinator::combinate_strings(decryptors.iter().map(|(id, _)| *id).collect()) {
        //let decrypted = at_bash.decrypt(str.clone(), i);
        if is_candidate(str.clone()) {
            //println!("CANDIDATE FOUND: {}", decrypted);
        }
    }
}

fn is_candidate(str: String) -> bool {
    str.contains("CLOCK") || str.contains("BERLIN") || str.contains("NORTH") || str.contains("EAST")
}

fn get_decryptors() -> Vec<(u8, Box<dyn Decryptor>)> {
    vec![
        (1, Box::new(atbash::AtBash {})),
        (2, Box::new(caesar::Caesar {})),
    ]
}
