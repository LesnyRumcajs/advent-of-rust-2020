use std::{
    io::{self, BufRead},
    str::FromStr,
};

use simple_error::SimpleError;

#[derive(PartialEq)]
enum ExecutionStatus {
    OK,
    BOOTLOOP,
    TERMINATED,
}

#[derive(Clone, Debug)]
enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split(' ').collect::<Vec<_>>();
        let val: i64 = chunks[1].parse().unwrap();
        match chunks[0] {
            "nop" => Ok(Instruction::NOP(val)),
            "acc" => Ok(Instruction::ACC(val)),
            "jmp" => Ok(Instruction::JMP(val)),
            _ => Err(SimpleError::new("unknown instruction!")),
        }
    }
}
#[derive(Clone)]
struct Program {
    accumulator: i64,
    next: usize,
    code: Vec<Instruction>,
    executed: Vec<usize>,
}

impl Program {
    fn from(lines: Vec<String>) -> Result<Self, SimpleError> {
        Ok(Program {
            accumulator: 0,
            next: 0,
            code: lines
                .iter()
                .map(|line| Instruction::from_str(line).unwrap())
                .collect(),
            executed: Vec::new(),
        })
    }

    fn tick(&mut self) -> ExecutionStatus {
        if self.executed.contains(&self.next) {
            return ExecutionStatus::BOOTLOOP;
        }

        if self.next >= self.code.len() {
            return ExecutionStatus::TERMINATED;
        }

        self.executed.push(self.next);
        match self.code[self.next] {
            Instruction::NOP(_) => {
                self.next += 1;
            }
            Instruction::JMP(val) => {
                self.next = (self.next as i64 + val) as usize;
            }
            Instruction::ACC(val) => {
                self.accumulator += val;
                self.next += 1;
            }
        };

        return ExecutionStatus::OK;
    }

    fn create_mutated(&self, index: usize) -> Result<Self, SimpleError> {
        match self.code[index] {
            Instruction::NOP(val) => {
                let mut clone = self.clone();
                clone.code[index] = Instruction::JMP(val);
                Ok(clone)
            }
            Instruction::JMP(val) => {
                let mut clone = self.clone();
                clone.code[index] = Instruction::NOP(val);
                Ok(clone)
            }
            Instruction::ACC(_) => Err(SimpleError::new("Cannot flip ACC")),
        }
    }
}

fn main() {
    let program = Program::from(read_program()).unwrap();
    println!("Day 8, part 1: {}", part1(program.clone()));
    println!("Day 8, part 2: {}", part2(program));
}

fn part1(mut program: Program) -> i64 {
    while program.tick() == ExecutionStatus::OK {}
    program.accumulator
}
fn part2(program: Program) -> i64 {
    let mut terminated = false;
    let mut line_to_flip = 0;
    let mut result = 0;
    while !terminated {
        if let Ok(mut program) = program.create_mutated(line_to_flip) {
            loop {
                match program.tick() {
                    ExecutionStatus::OK => (),
                    ExecutionStatus::BOOTLOOP => {
                        break;
                    }
                    ExecutionStatus::TERMINATED => {
                        terminated = true;
                        result = program.accumulator;
                        break;
                    }
                }
            }
        }
        line_to_flip += 1;
    }
    result
}

fn read_program() -> Vec<String> {
    io::stdin().lock().lines().filter_map(Result::ok).collect()
}
