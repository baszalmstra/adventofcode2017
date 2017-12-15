use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let f = File::open("inputs/day5.txt").expect("input file not found");
    
    let original_instructions = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut instructions = original_instructions.clone();
    let mut steps = 0;
    let mut index:i32 = 0;
    while index >= 0 && (index as usize) < instructions.len() {
        let prev_index = index as usize;
        index = index + instructions[index as usize];
        instructions[prev_index] += 1;
        steps += 1;
    }

    println!("Steps taken: {}", steps);

    let mut instructions = original_instructions;
    let mut steps = 0;
    let mut index:i32 = 0;
    while index >= 0 && (index as usize) < instructions.len() {
        let prev_index = index as usize;
        let offset = instructions[index as usize];
        index = index + offset;
        instructions[prev_index] += if offset >= 3 { -1 } else { 1 };
        steps += 1;
    }

     println!("Steps taken: {}", steps);
}
