#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
enum InstructionOperation {
    Increment,
    Decrement
}

#[derive(Debug)]
enum InstructionComparison {
    Larger,
    Smaller,
    LargerThan,
    SmallerThan,
    Equals,
    NotEquals
}

#[derive(Debug)]
struct Instruction {
    register: String,
    op:InstructionOperation,
    value: i32,
    cmp_register: String,
    cmp_op: InstructionComparison,
    cmp_value: i32
}

#[derive(Debug)]
enum InstructionParseError {
    RegexError,
    UnknownToken,
    ParseError(std::num::ParseIntError)
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Instruction, InstructionParseError> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^(\\w+)\\s(dec|inc)\\s(-?\\d+)\\sif\\s(\\w+)\\s(>=|>|<|<=|==|!=)\\s(-?\\d+)$").unwrap();
        }

        match RE.captures(s) {
            None            => Err(InstructionParseError::RegexError),
            Some(captures)  => {
                Ok(Instruction {
                    register: captures[1].to_owned(),
                    op: captures[2].parse::<InstructionOperation>()?,
                    value: captures[3].parse::<i32>()?,
                    cmp_register: captures[4].to_owned(),
                    cmp_op: captures[5].parse::<InstructionComparison>()?,
                    cmp_value: captures[6].parse::<i32>()?
                })
            }
        }
    }
}

impl FromStr for InstructionOperation {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<InstructionOperation, InstructionParseError> {
        match s {
            "inc"   => Ok(InstructionOperation::Increment),
            "dec"   => Ok(InstructionOperation::Decrement),
            _       => Err(InstructionParseError::UnknownToken),
        }
    }
}

impl FromStr for InstructionComparison {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<InstructionComparison, InstructionParseError> {
        match s {
            ">"     => Ok(InstructionComparison::Larger),
            "<"     => Ok(InstructionComparison::Smaller),
            ">="    => Ok(InstructionComparison::LargerThan),
            "<="    => Ok(InstructionComparison::SmallerThan),
            "=="    => Ok(InstructionComparison::Equals),
            "!="    => Ok(InstructionComparison::NotEquals),
            _       => Err(InstructionParseError::UnknownToken),
        }
    }
}

impl From<std::num::ParseIntError> for InstructionParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        InstructionParseError::ParseError(error)
    }
}

#[derive(Debug)]
struct Registers {
    values: HashMap<String, i32>
}

impl Registers {
    fn new() -> Registers {
        Registers {
            values: HashMap::new()
        }
    }

    fn value(&self, name:&str) -> i32 {
        match self.values.get(name) {
            Some(value) => *value,
            None        => 0
        }
    }

    fn valid(&self, instr:&Instruction) -> bool {
        let register_value = self.value(&instr.cmp_register);
        match instr.cmp_op {
            InstructionComparison::Larger       => register_value >  instr.cmp_value,
            InstructionComparison::Smaller      => register_value <  instr.cmp_value,
            InstructionComparison::LargerThan   => register_value >= instr.cmp_value,
            InstructionComparison::SmallerThan  => register_value <= instr.cmp_value,
            InstructionComparison::Equals       => register_value == instr.cmp_value,
            InstructionComparison::NotEquals    => register_value != instr.cmp_value,
        }
    }

    fn apply(&mut self, instr:&Instruction) {
        let register_value = self.value(&instr.register);
        let new_value = match instr.op {
            InstructionOperation::Increment => register_value + instr.value,
            InstructionOperation::Decrement => register_value - instr.value
        };
        self.values.insert(instr.register.clone(), new_value);
    }
}

fn main() {
    let f = File::open("inputs/day8.txt").expect("input file not found");
    let line_iter = BufReader::new(f).lines().map(|line| line.unwrap());
    let instructions = line_iter.map(|line| line.parse::<Instruction>().unwrap()).collect::<Vec<Instruction>>();
 
    
    
    let mut historic_max_value = 0;
    let mut registers = Registers::new();
    for instr in instructions {
        if registers.valid(&instr) {
            registers.apply(&instr);
            let cur_max_value = registers.values.values().max().unwrap();
            historic_max_value = historic_max_value.max(*cur_max_value)
        }
    }

    let max_value = registers.values.values().max().unwrap();

    println!("{:?}", registers);
    println!("Max value: {}", max_value);
    println!("Historic max value: {}", historic_max_value);
}