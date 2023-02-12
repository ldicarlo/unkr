- [ ] combinator
- [ ] cache
  - [ ] ./.cache/ENCRYPTEDSTRING
  - [ ] clue,clue2 1,2
- [ ] cryptors
  - [x] fold
  - [ ] enigma
  - [ ] indexcrypt (using previous cypher text to locate next char)
  - [ ] vigenere
- [ ] cut message in parts
- [ ] print better results (with values)

https://kryptosfan.wordpress.com/k3/k3-solution-3/

# Use Cases
- [ ] `bruteforce --clues clue,clue2 --string`
- [ ] `cargo run -- encrypt --string HELLOHELLO -- cut:5 vigenere:KEY transpose:12 swap:[0,5,6,4,2,3,1]`
- [ ] `cargo run -- encrypt --string HELLOHELLO --cryptors=cut:5,vigenere:1,transpose:12,swap:[0,5,6,4,2,3,1]`
