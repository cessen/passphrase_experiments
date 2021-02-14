use std::{borrow::Cow, str::FromStr};

use clap::{App, Arg};
use getrandom::getrandom;
use num_bigint::ToBigUint;

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
                .short("m")
                .long("max_word_length")
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
        .arg(
            Arg::with_name("diceware_list")
                .short("d")
                .long("diceware_list")
                .value_name("N")
                .help("Output a d(N) diceware list.  For example, for use with an 8-sided die, specify '8'")
                .takes_value(true)
                .validator(|s| {
                    usize::from_str(&s)
                        .and(Ok(()))
                        .or(Err("must be an integer".to_string()))
                }),
        )
        .arg(
            Arg::with_name("word_list_path")
                .short("w")
                .long("word_list")
                .value_name("N")
                .help("Path to an alternative word list to use")
                .takes_value(true),
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

    let word_list_text: Cow<str> = if let Some(path) = args.value_of("word_list_path") {
        std::fs::read_to_string(path)
            .expect("Could not read word list file.")
            .into()
    } else {
        WORD_LIST.into()
    };
    // let re = regex::Regex::new(r"^[a-z]+$").unwrap();
    let words: Vec<_> = word_list_text
        .lines()
        .filter_map(|s| {
            s.split_whitespace().last().and_then(|s| {
                if !s.is_empty() && s.len() <= max_word_length {
                    Some(s)
                } else {
                    None
                }
            })
        })
        .collect();

    //-----------------------------------------
    // If specified on the commandline, print a diceware list and exit.

    if let Some(base_str) = args.value_of("diceware_list") {
        let base = base_str.parse::<u32>().unwrap();
        let digit_count = (words.len() as f64).log(base as f64).ceil() as usize;
        let mut digits = vec![1u32; digit_count];

        for word in words.iter() {
            // Print dice rolls.
            for d in 0..digit_count {
                print!("{}", digits[digit_count - 1 - d]);
                if d < (digit_count - 1) {
                    print!("-");
                }
            }

            // Print word.
            println!("    {}", word);

            // Increment dice rolls.
            for d in 0..digit_count {
                digits[d] += 1;
                if digits[d] > base {
                    digits[d] = 1;
                } else {
                    break;
                }
            }
        }

        return;
    }

    //-----------------------------------------
    // Generate random passphrase.

    for _ in 0..passphrase_length {
        let i = secure_random_u32(0..(words.len() as u32)) as usize;
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

/// Number of bits of entropy of choosing an *ordered* set of `k` items
/// from a list of `source_size`.
fn perm_entropy(source_size: u64, k: u64) -> u64 {
    source_size.to_biguint().unwrap().pow(k as u32).bits() - 1
}

/// Number of bits of entropy of choosing an *unordered* set of `k` items
/// from a list of `source_size`.
///
/// This is a fairly straightforward application of the "multi-combination"
/// function:
/// https://en.wikipedia.org/wiki/Combination#Number_of_combinations_with_repetition
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

/// Generate a uniform random number in `range` from the OS's (hopefully)
/// secure source of random bytes.
///
/// This relies on the security of the bytes from `getrandom()`.
fn secure_random_u32(range: std::ops::Range<u32>) -> u32 {
    let mut bytes = [0u8; 16];
    getrandom(&mut bytes[..])
        .expect("Something went wrong with the secure random number generation.");
    let n = u128::from_ne_bytes(bytes);

    // Map the large u128 integer space to the smaller `range`.  Since the
    // space of u128 is so large, and since we're limiting ourselves at most
    // to a 32-bit output range, the bias incurred by using a simple modulus
    // is extremely small: at most `1 / 2^96`.  This should be beyond
    // sufficient for generating secure random passphrases.
    let count = (range.end - range.start) as u128;
    range.start + (n % count) as u32
}
