//! # Unkr decrypt and bruteforce old school cyphers
//!
//! - Encrypt and Decrypt strings using old school cryptography, like Vigenere, Transposition, and more.
//! - Bruteforce challenges when knowing a part of the clear text.
//! - Print out all possible strings and look for words inside yourself.
//!
//! ## Decrypt/Encrypt Examples
//!
//! ### Basic
//! ```bash
//! $ echo "ABCDEF" | unkr encrypt -- transpose:2
//! ACE
//! BDF
//! ```
//!
//! ### Kryptos panel K1
//!
//! ```bash
//! $ echo "EMUFPHZLRFAXYUSDJKZLDKRNSHGNFIVJYQTQUXQBQVYUVLLTREVJYQTMKYRDMFD" | unkr decrypt -- vigenere:PALIMPSEST:KRYPTOS
//! BETWEENSUBTLESHADINGANDTHEABSENCEOFLIGHTLIESTHENUANCEOFIQLUSION
//! ```
//!
//! ### Kryptos panel k2
//!
//! (passing string as argument instead of stdin)
//!
//! ```bash
//! $ unkr decrypt --string 'VFPJUDEEHZWETZYVGWHKKQETGFQJNCEGGWHKK?DQMCPFQZDQMMIAGPFXHQRLGTIMVMZJANQLVKQEDAGDVFRPJUNGEUNAQZGZLECGYUXUEENJTBJLBQCRTBJDFHRRYIZETKZEMVDUFKSJHKFWHKUWQLSZFTIHHDDDUVH?DWKBFUFPWNTDFIYCUQZEREEVLDKFEZMOQQJLTTUGSYQPFEUNLAVIDXFLGGTEZ?FKZBSFDQVGOGIPUFXHHDRKFFHQNTGPUAECNUVPDJMQCLQUMUNEDFQELZZVRRGKFFVOEEXBDMVPNFQXEZLGREDNQFMPNZGLFLPMRJQYALMGNUVPDXVKPDQUMEBEDMHDAFMJGZNUPLGEWJLLAETG' -- 'vigenere:ABSCISSA:KRYPTOS'
//! ITWASTOTALLYINVISIBLEHOWSTHATPOSSIBLE?THEYUSEDTHEEARTHSMAGNETICFIELDXTHEINFORMATIONWASGATHEREDANDTRANSMITTEDUNDERGRUUNDTOANUNKNOWNLOCATIONXDOESLANGLEYKNOWABOUTTHIS?THEYSHOULDITSBURIEDOUTTHERESOMEWHEREXWHOKNOWSTHEEXACTLOCATION?ONLYWWTHISWASHISLASTMESSAGEXTHIRTYEIGHTDEGREESFIFTYSEVENMINUTESSIXPOINTFIVESECONDSNORTHSEVENTYSEVENDEGREESEIGHTMINUTESFORTYFOURSECONDSWESTIDBYROWS
//! ```
//!
//! ### Kryptos panel k3
//!
//! There is some confusion here between encrypt and decrypt, I need to fix the right verb but I am not sure about what to for `transpose` actually.
//!
//! Also symbols are totally ignored.
//!
//!```bash
//! $ cargo run -- encrypt --string "ENDYAHROHNLSRHEOCPTEOIBIDYSHNAIACHTNREYULDSLLSLLNOHSNOSMRWXMNETPRNGATIHNRARPESLNNELEBLPIIACAEWMTWNDITEENRAHCTENEUDRETNHAEOETFOLSEDTIWENHAEIOYTEYQHEENCTAYCREIFTBRSPAMHHEWENATAMATEGYEERLBTEEFOASFIOTUETUAEOTOARMAEERTNRTIBSEDDNIAAHTTMSTEWPIEROAGRIEWFEBAECTDDHILCEIHSITEGOEAOSDDRYDLORITRKLMLEHAGTDHARDPNEOHMGFMFEUHEECDMRIPFEIMEHNLSSTTRTVDOHW?" -- transpose:24 reverse transpose:8 reverse join
//! SLOWLYDESPARATLYSLOWLYTHEREMAINSOFPASSAGEDEBRISTHATENCUMBEREDTHELOWERPARTOFTHEDOORWAYWASREMOVEDWITHTREMBLINGHANDSIMADEATINYBREACHINTHEUPPERLEFTHANDCORNERANDTHENWIDENINGTHEHOLEALITTLEIINSERTEDTHECANDLEANDPEEREDINTHEHOTAIRESCAPINGFROMTHECHAMBERCAUSEDTHEFLAMETOFLICKERBUTPRESENTLYDETAILSOFTHEROOMWITHINEMERGEDFROMTHEMISTXCANYOUSEEANYTHINGQ
//!```
//!
//! ### ASD Coin
//!
//! see https://www.asd.gov.au/news-events-speeches/events/2022-09-01-75th-anniversary-commemorative-coin
//!
//!```bash
//! $ echo "URMWXOZIRGBRM7DRWGSC5WVKGS" | cargo run -- encrypt -- atbash
//!
//! FINDCLARITYIN7WIDTHX5DEPTH
//!
//! $ echo "DVZIVZFWZXRLFHRMXLMXVKGZMWNVGRXFOLFHRMVCVXFGRLM" | cargo run -- encrypt -- atbash
//!
//! WEAREAUDACIOUSINCONCEPTANDMETICULOUSINEXECUTION
//!
//! $ echo "BGOAMVOEIATSIRLNGTTNEOGRERGXNTEAIFC" | cargo run -- encrypt -- transpose:7 join
//!
//! BELONGINGTOAGREATTEAMSTRIVINGFOREXC
//!
//! $ echo "ECAIEOALEKFNR5LWEFCHDEEAEEE7NMDRXX5" | cargo run -- encrypt -- transpose:7 join
//!
//! ELLENCEWEMAKEADIFFERENCEXORHEXA5D75
//! ```
//!
//!
//! ## Cryptors
//!
//! Current cryptors are
//! ```
//! # use unkr::models::{BruteForceCryptor, NumberArgs, PermuteArgs,BruteForceVigenereArgs,BruteForcePermuteArgs};
//! # let cryptors = unkr::get_decryptors();
//! assert_eq!(cryptors,  vec![
//!        BruteForceCryptor::Vigenere(BruteForceVigenereArgs { // [...]
//!  #          alphabet_depth: 1,
//!  #          key_depth: 2,
//!        }),
//!        BruteForceCryptor::Cut,
//!        BruteForceCryptor::Caesar,
//!        BruteForceCryptor::Transpose,
//!        BruteForceCryptor::AtBash,
//!        BruteForceCryptor::Reverse,
//!        BruteForceCryptor::Swap,
//!        BruteForceCryptor::Join,
//!        BruteForceCryptor::Permute(BruteForcePermuteArgs { // [...]
//!  #          max_permutations: 2,
//!        }),
//!    ]);
//! ```
//!
//! ## Bruteforce
//!
//! The second interesting feature is bruteforcing using clues.
//!
//! ```bash
//! cargo run -- bruteforce --decryptors enigma --string ILBDARKFH --clues HELLOTEST --threads 16
//!
//! ```

use cryptors::{
    char_utils,
    enigma::{self, EnigmaArgs},
    permute, transpose,
};
use models::{BruteForceCryptor, NumberArgs, PermuteArgs};
mod base;
mod brute_force;
mod brute_force_state;
mod cache;
mod candidates;
mod colorize;
mod combinator;
mod console;
mod cryptors;
mod decrypt;
mod decryptors;
mod encrypt;
mod fuzzer;
mod mapper;
// pub for benchmark, but not sure that it is really required
pub mod models;
mod parser;
mod thread_system;

pub fn fuzz_next_string_ruled(
    str: &String,
    len_max: usize,
    base: usize,
    unique_letters_constraint: bool,
    pair_length_constraint: bool,
    sorted_by_pair_constraint: bool,
) -> Option<String> {
    fuzzer::fuzz_next_string_ruled(
        str,
        len_max,
        base,
        unique_letters_constraint,
        pair_length_constraint,
        sorted_by_pair_constraint,
    )
}

pub fn print_encrypt(strs: Vec<String>, decryptors: Vec<String>) {
    encrypt::print_encrypt(strs, decryptors)
}

pub fn print_decrypt(str: Vec<String>, decryptors: Vec<String>) {
    decrypt::print_decrypt(str, decryptors)
}

pub fn brute_force_decrypt(
    str: String,
    clues: Vec<String>,
    steps: u8,
    decryptors: Vec<String>,
    threads_numbers: Vec<u8>,
    threads_count: u8,
    pretty: bool,
    cache_name: String,
) {
    brute_force::brute_force_decrypt(
        str,
        clues,
        steps,
        decryptors,
        threads_numbers,
        threads_count,
        pretty,
        cache_name,
    )
}

/// Return current supported decryptors.
pub fn get_decryptors() -> Vec<BruteForceCryptor> {
    decryptors::get_decryptors()
}

pub fn brute_force_unique_combination(
    str: String,
    clues: Vec<String>,
    decryptors: Vec<String>,
    threads_number: Vec<u8>,
    total_threads: u8,
    cache_name: String,
    pretty: bool,
    intermediate_steps: bool,
) {
    brute_force::brute_force_unique_combination(
        str,
        clues,
        decryptors,
        threads_number,
        total_threads,
        cache_name,
        pretty,
        intermediate_steps,
    )
}

pub fn print_combine_elements(elements_count: u8, picks: u8) {
    combinator::print_combine_elements(elements_count, picks)
}

pub fn read_bruteforce_parameters(str: String) -> BruteForceCryptor {
    parser::read_bruteforce_parameters(str)
}

pub fn fuzz_from(str: String, len_max: usize, base: usize, rules: Vec<String>) {
    fuzzer::fuzz_from(str, len_max, base, rules)
}

pub fn fuzz_next(str: &Vec<u8>, len_max: usize, base: usize) -> Option<Vec<u8>> {
    fuzzer::fuzz_next(str, len_max, base, &(base as u8 - 1))
}

pub fn enigma_next(enigma_args: EnigmaArgs) -> Option<EnigmaArgs> {
    enigma::next(enigma_args)
}

pub fn enigma_init() -> EnigmaArgs {
    enigma::init()
}

pub fn enigma_encrypt(strs: Vec<String>, enigma_args: EnigmaArgs) -> Vec<String> {
    enigma::encrypt(strs, enigma_args)
}

pub fn transpose_init() -> NumberArgs {
    transpose::init()
}

pub fn transpose_next(strs: Vec<String>, number_args: NumberArgs) -> Option<NumberArgs> {
    transpose::next(strs, number_args)
}

pub fn transpose_decrypt(strs: Vec<String>, number_args: NumberArgs) -> Vec<String> {
    transpose::decrypt(strs, number_args)
}

pub fn permute_init() -> PermuteArgs {
    permute::init()
}

pub fn permute_next(
    permute_brute_force_state: models::PermuteBruteForceState,
) -> Option<models::PermuteArgs> {
    permute::next(permute_brute_force_state)
}

pub fn permute_decrypt(strs: Vec<String>, permute_args: PermuteArgs) -> Vec<std::string::String> {
    permute::decrypt(strs, permute_args)
}

pub fn char_position(c: char) -> usize {
    char_utils::char_position_base(c)
}
