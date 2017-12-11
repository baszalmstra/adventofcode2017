#![feature(io)]

use std::io::Read;
use std::fs::File;

fn main() {
    // Read the input file
    let f = File::open("inputs/day1.txt").expect("input file not found");

    // Convert all characters to integer values
    let values:Vec<u32> = f.chars()
        .map(|c| char::to_digit(c.unwrap(), 10).unwrap())
        .collect();

    // Filter all elements that are not applicable
    let valid_values:Vec<u32> = values.iter().enumerate().filter_map(|(i, item)| {
        if values[(i+1)%values.len()] == *item { Some(*item) } else { None }
    }).collect();

    // Take the sum of all elements in the array
    let sum:u32 = valid_values.iter().sum();

    // Filter all elements that are not applicable for part two
    let offset = values.len()/2;
    let valid_values:Vec<u32> = values.iter().enumerate().filter_map(|(i, item)| {
        if values[(i+offset)%values.len()] == *item { Some(*item) } else { None }
    }).collect();

    // Take the sum of all elements in the array
    let part_two_sum:u32 = valid_values.iter().sum();
    println!("-- Day 1");
    println!("Part two: {}", sum);    
    println!("Part one: {}", part_two_sum);
}
