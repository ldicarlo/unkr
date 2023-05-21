- [x] `combinator.rs`
  - [x] clue,clue2 1,2
- [ ] cryptors
  - [x] fold
  - [?] enigma
  - [x] indexcrypt
  - [x] vigenere
    - [x] next impl
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
- [x] read from stdin by default
  - [x] encrypt
  - [x] decrypt
- [ ] always test:
  - [x] all cryptors encrypt(decrypt())
  - [ ] all cryptors readable
- [ ] decryptors docs
- [ ] bruteforce finishers?
  - [ ] colors
  - [ ] join
  - [ ] bold
- [x] useless combination (join join)
- [ ] NumberArgs -> usize
- [ ] Division dispatcher threads
- [ ] remove join?
- [ ] ASD coin
- [ ] numbers in chars ??
- [ ] fuzzer rules in CLI params

# Use Cases
- [x] `cargo run -- bruteforce --string STRING --clue clue --clue clue2 `
- [x] use enhanced / encrypt methods when useful
  - [x] `cargo run -- encrypt --string HELLOHELLO -- cut:5 vigenere:KEY:ALPHABET transpose:12 swap:0:5:6:4:2:3:1`
- [ ] `unkr decryptor <decryptor>` shows doc for decryptor
- [ ] `unkr bruteforce --clues-file ./words`

https://kryptosfan.wordpress.com/k3/k3-solution-3/
http://kryptools.com/hints.htm

# Caching

- [ ] save combinations in the form `./md5string/md5clues/{hits,done}`:
  - [ ] hits file line `vigenere:ABC:DEF join;RESULT`
  - [ ] done file line `vigenere join cut;vigenere:3:3` (only applying params)
  - [ ] (always save in done)

# Perf

- [ ] use a multiproducer channel to get hits ?
