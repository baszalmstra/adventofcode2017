use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn checksum(column:&Vec<u32>) -> Option<u32> {
    let max_value = column.iter().max()?;
    let min_value = column.iter().min()?;
    Some(max_value - min_value)
}

fn even_division(column:&Vec<u32>) -> Option<u32> {
    for val in column {
        for divisor in column {
            if val != divisor && val % divisor == 0 {
                return Some(val/divisor)
            }
        }
    }
    None
}

fn main() {
    let f = File::open("inputs/day2.txt").expect("input file not found");

    let columns:Vec<Vec<u32>> =
        // Read all lines
        BufReader::new(f).lines()
        // Map lines to columns of integerss
        .map(|line| line.unwrap().split("\t").map(|word| word.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        // And give me an array of columns
        .collect::<Vec<Vec<u32>>>();
    
    let checksum:u32 = columns.iter()
        // Calculate the checksum of every column
        .map(|column| checksum(&column).unwrap())
        // Sum all the checksums
        .sum();

    let evenly_divisible_values = columns.iter()
        // Calculate the checksum of every column
        .map(|column| even_division(&column).unwrap())
        // Sum all the checksums
        .sum::<u32>();  

    println!("Part 1: {:?}", checksum);
    println!("Part 2: {:?}", evenly_divisible_values);
}