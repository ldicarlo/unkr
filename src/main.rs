mod atbash;
mod caesar;
mod combinator;
mod decryptors;
mod fold;
use decryptors::Decryptor;
use std::collections::BTreeMap;
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

fn decrypt(
  mut decryptors: Vec<(u8, Box<dyn Decryptor>)>,
  mut used: Vec<u8>,
  mut to_use: Vec<u8>,
  str: String,
) -> bool {
  if let Some(current) = used.pop() {
    let (_, decryptor) = decryptors
      .into_iter()
      .find(|(id, _)| *id == current)
      .unwrap();
    let newStr = decryptor.decrypt(str, 1);
    decrypt(decryptors.clone(), used, to_use, newStr)
  } else {
    false
  }
}

fn is_candidate(str: String) -> bool {
  str.contains("CLOCK") || str.contains("BERLIN") || str.contains("NORTH") || str.contains("EAST")
}

fn get_decryptors<F>() -> Vec<(u8, F)>
where
  F: Fn() -> u64,
{
  vec![
    (1, (|| atbash::get_max_seed())),
    (2, (|| caesar::get_max_seed())),
  ]
}
