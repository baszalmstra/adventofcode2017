#![feature(entry_and_modify)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

type Passphrase = Vec<String>;

fn is_valid(passphrase:&Passphrase) -> bool{
    let mut words = HashSet::new();
    for word in passphrase {
        if words.contains(word) {
            return false;
        }
        words.insert(word);
    }
    true
}

fn is_valid_anagrams(passphrase:&Passphrase) -> bool{
    let mut words = Vec::new();
    for word in passphrase {
        let mut chars = HashMap::new();
        for c in word.chars() {
            chars.entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        if words.contains(&chars) {
            return false;
        }
        words.push(chars);
    }
    true
}

fn main() {
    let f = File::open("inputs/day4.txt").expect("input file not found");

    let passphrases = BufReader::new(f).lines()
        .map(|line| line.unwrap().split(" ").map(|word| String::from(word)).collect::<Passphrase>())
        .collect::<Vec<Passphrase>>();

    let valid_passphrases = passphrases.iter()
        .filter(|phrase| is_valid(&phrase))
        .count();

    let valid_anagram_passphrases = passphrases.iter()
        .filter(|phrase| is_valid_anagrams(&phrase))
        .count();
    
    println!("Valid phrases {}/{}", valid_passphrases, passphrases.len());
    println!("Valid anagram phrases {}/{}", valid_anagram_passphrases, passphrases.len());
}