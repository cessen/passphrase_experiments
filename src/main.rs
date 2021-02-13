use std::str::FromStr;

use clap::{App, Arg};
use num_bigint::ToBigUint;
use rand::{rngs::OsRng, Rng};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const WORD_LIST: &str = include_str!("../word_list.txt");

fn main() {
    // Parse command line arguments.
    let args = App::new("passphrase")
        .version(VERSION)
        .about("Generates random passphrases.")
        .arg(
            Arg::with_name("passphrase_length")
                .short("l")
                .long("length")
                .value_name("N")
                .help("Length of the generated passphrase in words")
                .takes_value(true)
                .default_value("5")
                .validator(|s| {
                    usize::from_str(&s)
                        .and(Ok(()))
                        .or(Err("must be an integer".to_string()))
                }),
        )
        .arg(
            Arg::with_name("max_word_length")
                .short("w")
                .long("word_length")
                .value_name("N")
                .help("Max word length in characters")
                .takes_value(true)
                .default_value("7")
                .validator(|s| {
                    usize::from_str(&s)
                        .and(Ok(()))
                        .or(Err("must be an integer".to_string()))
                }),
        )
        .get_matches();

    //-----------------------------------------
    // Get command line arguments.

    let passphrase_length = if let Some(n) = args.value_of("passphrase_length") {
        usize::from_str(n).unwrap()
    } else {
        unreachable!() // Should never happen due to the default value.
    };

    let max_word_length = if let Some(n) = args.value_of("max_word_length") {
        usize::from_str(n).unwrap()
    } else {
        unreachable!() // Should never happen due to the default value.
    };

    //-----------------------------------------
    // Parse word list.

    let re = regex::Regex::new(r"^[a-z]+$").unwrap();
    let words: Vec<_> = WORD_LIST
        .lines()
        .map(|s| s.trim())
        .filter(|s| re.is_match(s) && s.len() <= max_word_length)
        .collect();

    //-----------------------------------------
    // Generate random passphrase.

    for _ in 0..passphrase_length {
        let i = OsRng.gen_range(0..words.len());
        print!("{} ", words[i]);
    }

    println!(
        "\n\nEntropy in this order:  {} bits",
        perm_entropy(words.len() as u64, passphrase_length as u64)
    );
    println!(
        "Entropy rearranged:     {} bits",
        cmbn_entropy(words.len() as u64, passphrase_length as u64)
    );
    println!("Source word count:      {}", words.len() as u64);
}

fn perm_entropy(source_size: u64, k: u64) -> u64 {
    source_size.to_biguint().unwrap().pow(k as u32).bits() - 1
}

/// The multi-combination function, returning the bits of entropy of a given choice.
fn cmbn_entropy(source_size: u64, k: u64) -> u64 {
    if k == 0 {
        return 0;
    }

    let mut n = (source_size + k - 1).to_biguint().unwrap();
    let mut c = n.clone();
    for i in 2..(k + 1) {
        n -= 1u64;
        c *= n.clone();
        c /= i;
    }

    c.bits() - 1
}
