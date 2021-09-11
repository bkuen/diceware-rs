//! This rust application generates diceware password.
//! By default, german passwords with 7 words are generated.
//!
//! Read more [here](https://theintercept.com/2015/03/26/passphrases-can-memorize-attackers-cant-guess/)

use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::io;

const DICEWARE_URL: &'static str = "https://theworld.com/~reinhold/diceware_german.txt";

/// Reads in a list of diceware key-value pairs and returns
/// a `HashMap` mapping a number to a corresponding word
fn read_diceware_list() -> HashMap<usize, String> {
    reqwest::blocking::get(DICEWARE_URL)
        .expect("Failed to read diceware list")
        .text()
        .expect("Could not read diceware list")
        .split("\n")
        .filter(|&s| s.len() != 0)
        .map(|s| s.trim().split_at(s.find(" ").unwrap()))
        .map(|(key, val)| (key.parse::<usize>().unwrap(), val[1..].to_owned()))
        .collect()
}

/// Returns a random 5 digit long number using `rand`
/// create.
///
/// # Arguments
///
/// * `rng` - A mutable instance to `rand::prelude::ThreadRng`
fn gen_rnd_number(rng: &mut ThreadRng) -> usize {
    let mut number = 0;
    for i in 0..5 {
        let n: usize = rng.gen_range(1..9);
        number += n * 10usize.pow(i);
    }
    number
}


/// Returns a passphrase of length `len` using diceware algorithm
///
/// # Arguments
///
/// * `map` - A reference to a diceware key-value map
/// * `len` - The amount of words of the passphrase
fn gen_diceware(map: &HashMap<usize, String>, len: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| {
            let word;
            loop {
                let rnd = gen_rnd_number(&mut rng);
                match map.get(&rnd) {
                    None => continue,
                    Some(w) => {
                        word = w.as_str();
                        break;
                    }
                }
            }
            word
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    println!("Load diceware list");
    let map = read_diceware_list();

    let mut repeat = String::new();
    loop {
        println!("New diceware? [y/n]");

        io::stdin().read_line(&mut repeat)
            .ok()
            .expect("Failed to read line");

        if repeat == "n" {
            break;
        }

        let diceware = gen_diceware(&map, 7);
        println!("Generated diceware: \n{}", diceware);
    }
}
