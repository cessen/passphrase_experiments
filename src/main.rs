use std::{borrow::Cow, convert::TryInto, str::FromStr};

use clap::{App, Arg};
use getrandom::getrandom;
use num_bigint::{BigUint, ToBigUint};

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
                .value_name("WORDS")
                .help("Desired passphrase length in words")
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
                .value_name("CHARS")
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
            Arg::with_name("pick_from")
                .short("p")
                .long("pick_from")
                .value_name("N")
                .help("Generate N words, and let the user pick <WORDS> words from them")
                .takes_value(true)
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
                .value_name("PATH")
                .help("Specify an alternative word list to use")
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

    let word_choice_set_size = if let Some(n) = args.value_of("pick_from") {
        let size = usize::from_str(n).unwrap();
        if size < passphrase_length {
            println!("Oops: the number of words to pick from (you passed '{}') must be equal to or greater than the passphrase length (you passed '{}').", size, passphrase_length);
            return;
        }
        size
    } else {
        passphrase_length
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
    // Print random passphrase, instructions, and info.

    for _ in 0..word_choice_set_size {
        let i = secure_random_u32(0..(words.len() as u32)) as usize;
        print!("{} ", words[i]);
    }
    println!();

    if word_choice_set_size > passphrase_length {
        println!(
            "\n^ Pick {} words from this list, and\n  arrange them however you like.",
            passphrase_length
        );

        println!(
            "\nMin-entropy:        {} bits",
            (multi_cmbns(words.len(), word_choice_set_size)
                / multi_cmbns(words.len(), word_choice_set_size - passphrase_length))
            .bits()
            .saturating_sub(1)
        );
    } else {
        println!(
            "\n^ Rearrange these {} words however you like.",
            passphrase_length
        );

        println!(
            "\nMin-entropy:        {} bits",
            multi_cmbns(words.len(), passphrase_length)
                .bits()
                .saturating_sub(1)
        );
    }

    println!("Source word count:  {}", words.len());
}

/// Calculates the number of combinations with repetition (also called
/// multi-combinations).
///
/// See:
/// https://en.wikipedia.org/wiki/Combination#Number_of_combinations_with_repetition
fn multi_cmbns(source_size: usize, k: usize) -> BigUint {
    if k == 0 {
        return 0.to_biguint().unwrap();
    }

    let mut n = (source_size + k - 1).to_biguint().unwrap();
    let mut c = n.clone();
    for i in 2..(k + 1) {
        n -= 1u64;
        c *= n.clone();
        c /= i;
    }

    c
}

/// Generate a uniform random number in `range` from the OS's (hopefully)
/// secure source of random bytes.
///
/// This relies on the security of the bytes from `getrandom()`.
fn secure_random_u32(range: std::ops::Range<u32>) -> u32 {
    let mut bytes = [0u8; 36]; // 256 + 32 bits
    getrandom(&mut bytes[..]).expect("Failed to generate random numbers.");
    let n = BigUint::from_bytes_le(&bytes);

    // Map the large 288-bit integer space to the smaller `range`.  Since the
    // space of the large int is so big, and since we're limiting ourselves at
    // most to a 32-bit output range, the bias incurred by using a simple
    // modulus is extremely small: at most `1 / 2^256`.  This should be beyond
    // sufficient for generating secure random passphrases.
    let count = (range.end - range.start).to_biguint().unwrap();
    let offset: u32 = (n % count).try_into().unwrap();
    range.start + offset
}
