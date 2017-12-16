use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Memory {
    banks: [u8; 16],
}

impl Memory {
    fn cycle(&self)->Memory {
        // Find the highest value
        let (index, value) = self.banks.iter().enumerate()
            .fold((0,0), |(cur_idx, cur), (idx, &val)| {
                if cur < val { (idx, val) } else { (cur_idx, cur) }
            });
        let offset:usize = index as usize;

        // Redistribute the value
        let mut new_banks = self.banks.clone();
        new_banks[offset] = 0u8;
        for i in 0..value {
            new_banks[(offset+(i as usize)+1)%16] += 1;
        }

        Memory { banks: new_banks }
    }
}

fn main() {
    let f = File::open("inputs/day6.txt").expect("input file not found");

    let line = BufReader::new(f).lines().nth(0).unwrap().unwrap();
    let mut memory = Memory { banks: [0; 16]};
    let banks = line.split("\t").map(|bank| bank.parse::<u8>().unwrap());
    let mut i = 0;
    for bank in banks {
        memory.banks[i] = bank;
        i += 1;
    }
    
    let mut seen = HashSet::new();
    seen.insert(memory);
    let mut i = 0;
    loop {
        memory = memory.cycle();
        i += 1;
        if seen.contains(&memory) {
            break;
        }
        seen.insert(memory);
    }

    println!("Solution #1: {}", i);

    i = 0;
    let cycle = memory;
    loop {
        memory = memory.cycle();
        i += 1;
        if memory == cycle {
            break;
        }
    }

    println!("Solution #2: {}", i);
}