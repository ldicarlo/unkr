- [x] `combinator.rs`
- [ ] cache
  - [ ] ./.cache/ENCRYPTEDSTRING.md5
  - [x] clue,clue2 1,2
- [ ] cryptors
  - [x] fold
  - [?] enigma
  - [x] indexcrypt
  - [x] vigenere
    - [ ] next impl
  - [ ] asciimorse?
  - [x] permutations
- [x] cut message in parts
- [ ] print better results (with values)
- [x] finish deserialization for cryptors
- [ ] document shit
- [x] harmonize method names
- [ ] fix test method names
- [ ] perfs monitoring
- [x] multithreading
- [ ] profiling
- [ ] when a thread is done, reallocate from another
- [x] move to `get_next`
- [ ] read from stdin `--stdin`
  - [ ] bruteforce
  - [ ] encrypt
  - [ ] decrypt
- [ ] always test:
  - [x] all cryptors encrypt(decrypt())
  - [ ] all cryptors readable
- [ ] decryptors docs
- [ ] bruteforce finishers?
  - [ ] colors
  - [ ] join
  - [ ] bold
- [ ] useless combination (join join)
- [ ] NumberArgs -> usize


# Use Cases
- [x] `cargo run -- bruteforce --string STRING --clue clue --clue clue2 `
- [x] use enhanced / encrypt methods when useful
  - [x] `cargo run -- encrypt --string HELLOHELLO -- cut:5 vigenere:KEY:ALPHABET transpose:12 swap:0:5:6:4:2:3:1`
- [ ] `unkr decryptor <decryptor>` shows doc for decryptor
- [ ] `unkr bruteforce --clues-file ./words`

https://kryptosfan.wordpress.com/k3/k3-solution-3/
http://kryptools.com/hints.htm
