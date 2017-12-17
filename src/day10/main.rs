use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter;
use std::io::Read;

type Arr = [u32; 256];

fn reverse(a:Arr, from: u32, len:u32) -> Arr {
    let mut result = a.clone();
    for i in 0..len {
        result[((i+from) % 256) as usize] = a[((from+len-i-1) as usize)%256];
    }
    result
}

fn main() {
    let f = File::open("inputs/day10.txt").expect("input file not found");
    let lengths = BufReader::new(f).lines().next().unwrap().unwrap().split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut arr = [0u32; 256];
    for i in 0..256 {
        arr[i] = i as u32;
    }

    let mut pos = 0;
    let mut skip_size = 0;
    for len in lengths {
        arr = reverse(arr, pos, len);
        pos = (pos + len + skip_size) % 256;
        skip_size += 1;
    }

    println!("{}", arr[0]*arr[1]);

    let f = File::open("inputs/day10.txt").expect("input file not found");
    let mut lengths = BufReader::new(f).bytes().map(|c| c.unwrap() as u32).collect::<Vec<u32>>();
    let mut default_seq = vec![17, 31, 73, 47, 23];
    lengths.append(&mut default_seq);

    let mut arr = [0u32; 256];
    for i in 0..256 {
        arr[i] = i as u32;
    }

    let mut pos = 0;
    let mut skip_size = 0;
    for round in 0..64 {
        for len in lengths.iter() {
            arr = reverse(arr, pos, *len);
            pos = (pos + len + skip_size) % 256;
            skip_size += 1;
        }
    }

    let mut dense_hash = [0u32; 16];
    for block in 0..16 {
        let start = block*16;
        let mut value = arr[start];
        for i in start+1..start+16 {
            value ^= arr[i];
        }
        dense_hash[block] = value;
    }

    println!("{}", dense_hash.iter().map(|c| format!("{:02x}", *c)).collect::<String>());
}