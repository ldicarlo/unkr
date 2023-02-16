- [x] `combinator.rs`
- [ ] cache
  - [ ] ./.cache/ENCRYPTEDSTRING
  - [x] clue,clue2 1,2
- [x] cryptors
  - [x] fold
  - [?] enigma
  - [x] indexcrypt
  - [x] vigenere
- [x] cut message in parts
- [ ] print better results (with values)
- [x] finish deserialization for cryptors
- [ ] document shit
- [ ] harmonize method names
- [ ] fix test method names
- [ ] perfs monitoring
- [ ] multithreading
- [ ] profiling


# Use Cases
- [x] `cargo run -- bruteforce --string STRING -- clue,clue2 `
- [x] use enhanced / encrypt methods when useful
  - [x] `cargo run -- encrypt --string HELLOHELLO -- cut:5 vigenere:KEY:ALPHABET transpose:12 swap:0:5:6:4:2:3:1`


https://kryptosfan.wordpress.com/k3/k3-solution-3/
