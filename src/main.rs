mod atbash;
mod caesar;
mod combinator;
mod fold;
fn main() {
  let str = "OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR".to_string().to_uppercase();
  let decryptors = get_decryptors();

  for i in combinator::combinate_strings(decryptors.iter().map(|(id, _)| *id).collect()) {
    if let Some(result) = loop_decrypt(vec![], i, str.clone()) {
      println!("CANDIDATE FOUND: {:?}", result);
    }
  }
}

fn loop_decrypt(used: Vec<u8>, mut to_use: Vec<u8>, str: String) -> Option<Vec<u8>> {
  if let Some(current) = to_use.pop() {
    let (_, (seed, decrypt)) = get_decryptors()
      .into_iter()
      .find(|(id, _)| *id == current)
      .unwrap();
    let new_str = decrypt(str, 1);
    if is_candidate(new_str.clone()) {
      Some(used)
    } else {
      loop_decrypt(used, to_use, new_str.clone())
    }
  } else {
    None
  }
}

fn is_candidate(str: String) -> bool {
  str.contains("CLOCK") || str.contains("BERLIN") || str.contains("NORTH") || str.contains("EAST")
}

fn get_decryptors() -> Vec<(
  u8,
  (Box<dyn Fn() -> u64>, Box<dyn Fn(String, u64) -> String>),
)> {
  vec![
    (
      1,
      (Box::new(atbash::get_max_seed), Box::new(atbash::decrypt)),
    ),
    (
      2,
      (Box::new(caesar::get_max_seed), Box::new(caesar::decrypt)),
    ),
  ]
}
