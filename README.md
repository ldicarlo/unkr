# Unkr - see documentation at crate.io

## Use Cases
- [x] `cargo run -- bruteforce --string STRING --clue clue --clue clue2 `
- [x] use enhanced / encrypt methods when useful
  - [x] `cargo run -- encrypt --string HELLOHELLO -- cut:5 vigenere:KEY:ALPHABET transpose:12 swap:0:5:6:4:2:3:1`
- [ ] `unkr decryptor <decryptor>` shows doc for decryptor
- [ ] `unkr bruteforce --clues-file ./words`
- [x] `unkr bruteforce-combination --clue clue --string string -- permute:4 vigenere:2:3`

https://kryptosfan.wordpress.com/k3/k3-solution-3/
http://kryptools.com/hints.htm

## Caching

- [x] save combinations in the form `./md5string/md5clues/{hits,done}`:
  - [x] hits file line `vigenere:ABC:DEF join;RESULT`
  - [x] done file line `vigenere join cut;vigenere:3:3` (only applying params)
  - [x] (always save in done)
- [ ] partial cache
  - [ ] Partial BruteForce
    - [ ] First I do Permute length=3, then length=5 -> cache applies
  - [ ] Partial Checkpoint
    - [ ] When using Ctrl + C, write all current permutations to cache as
      `vigenere join cut;vigenere:3:3;partial vigenere:ABC:ALPHABET join cut:3`
  - [ ] `vigenere join cut;vigenere:3:3`

## Perf

- [x] use a multiproducer channel to get hits ?
- [ ] https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html

## TODO

- [x] `combinator.rs`
  - [x] clue,clue2 1,2
- [ ] cryptors
  - [x] fold
  - [x] enigma
    - [x] `echo HELLOTEST | cargo run -- decrypt -- enigma:B::I:0:II:0:III:0`
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
- [~] profiling
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
- [ ] use EnumIter everywhere ?
- [ ] https://github.com/ashvardanian/StringZilla

