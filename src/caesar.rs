pub fn get_max_seed() -> u64 {
  26
}

pub fn decrypt(str: String, seed: u64) -> String {
  str
    .chars()
    .into_iter()
    .map(|c| char_mod(c, seed.try_into().unwrap()))
    .collect()
}

fn char_mod(c: char, number: usize) -> char {
  let alphabet = vec![
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];
  alphabet
    .binary_search(&c)
    .ok()
    .and_then(|index| alphabet.get((index + number) % 26))
    .map(|ch| *ch)
    .unwrap_or(c)
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(char_mod('A', 5), 'F')
  }
  #[test]
  fn it_works_2() {
    assert_eq!(decrypt("MYNAMEISCAESAR".to_string(), 10), "WIXKWOSCMKOCKB")
  }
}
