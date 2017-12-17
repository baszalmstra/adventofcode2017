#![feature(io)]

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn read_garbage<T>(iter: &mut T) -> u32
    where T: Iterator<Item=char>,
{
    let mut score = 0;
    while let Some(c) = iter.next() {
        match c {
            '>' => {break;},
            '!' => {iter.next();},
            _   => {score += 1;}
        };
    }
    score
}

fn read_group<T>(iter:&mut T, self_score: u32) -> (u32, u32) 
    where T: Iterator<Item=char>,
{
    let mut score = self_score;
    let mut garbage_score = 0;
    while let Some(c) = iter.next() {
        match c {
            '}'         => {break;},
            '{'         => {let (s,g) = read_group(iter, self_score + 1); score += s; garbage_score += g;},
            '<'         => {garbage_score += read_garbage(iter);},
            _           => {},

        };
    }
    (score, garbage_score)
}

fn main() {
    let f = File::open("inputs/day9.txt").expect("input file not found");
    let reader = BufReader::new(f);
    let mut char_iter = reader.chars().map(|c| c.unwrap());
    let mut test = "{{{}}}".chars();
    
    char_iter.next();
    let (score, garbage_score) = read_group(& mut char_iter, 1);
    println!("Score: {}, garbage: {}", score, garbage_score);
}