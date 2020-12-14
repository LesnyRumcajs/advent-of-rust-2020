use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 14, part 1: {}", part1(&input));
    println!("Day 14, part 2: {}", part2(&input));
}

fn part1(input: &[ProgramChunk]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for chunk in input.iter() {
        for instruction in chunk.instructions.iter() {
            mem.insert(
                instruction.0,
                (instruction.1 & chunk.bitmask_zeroes) | chunk.bitmask_ones,
            );
        }
    }
    mem.values().sum()
}

fn part2(input: &[ProgramChunk]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for chunk in input.iter() {
        for instruction in chunk.instructions.iter() {
            for variation in apply_with_variations(instruction.0, &chunk.original_bitmask) {
                mem.insert(variation, instruction.1);
            }
        }
    }
    mem.values().sum()
}

#[derive(Default, Debug)]
struct ProgramChunk {
    original_bitmask: Vec<u8>,
    bitmask_zeroes: u64,
    bitmask_ones: u64,
    instructions: Vec<(u64, u64)>,
}

fn apply_with_variations(num: u64, bitmask: &[u8]) -> Vec<u64> {
    let num_str = format!("{:036b}", num);

    let intermediary = num_str
        .chars()
        .zip(bitmask)
        .map(|(ch, b)| match *b as char {
            '0' => ch as u8,
            '1' => b'1',
            'X' => b'X',
            _ => panic!("fiasco"),
        })
        .collect::<Vec<_>>();

    bitmask_variation(&intermediary)
}

fn bitmask_variation(bitmask: &[u8]) -> Vec<u64> {
    let mut result = Vec::new();
    if let Some(pos) = bitmask.iter().position(|&ch| ch == b'X') {
        let mut new_bitmask = bitmask.to_owned();
        new_bitmask[pos] = b'0';
        result.append(&mut bitmask_variation(&new_bitmask));
        new_bitmask[pos] = b'1';
        result.append(&mut bitmask_variation(&new_bitmask));
    } else {
        let mut bitmask_ones = 0u64;
        for b in bitmask {
            match *b as char {
                '0' => {
                    bitmask_ones <<= 1;
                }
                '1' => {
                    bitmask_ones = bitmask_ones << 1 | 1;
                }
                _ => panic!("fiasco input"),
            };
        }
        result.push(bitmask_ones);
    };

    result
}

fn read_input<R: BufRead>(reader: R) -> Vec<ProgramChunk> {
    let mut result = Vec::new();
    let mut current_chunk = ProgramChunk::default();

    for line in reader.lines().filter_map(Result::ok) {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = (\w+)$").unwrap();
            static ref ASSIGNMENT_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        if let Some(caps) = MASK_RE.captures(&line) {
            if !current_chunk.instructions.is_empty() {
                result.push(current_chunk);
            }

            let mut bitmask_zeroes = 0u64;
            let mut bitmask_ones = 0u64;
            for ch in caps[1].chars() {
                match ch {
                    'X' => {
                        bitmask_zeroes = bitmask_zeroes << 1 | 1;
                        bitmask_ones <<= 1;
                    }
                    '0' => {
                        bitmask_zeroes <<= 1;
                        bitmask_ones <<= 1;
                    }
                    '1' => {
                        bitmask_zeroes <<= 1;
                        bitmask_ones = bitmask_ones << 1 | 1;
                    }
                    _ => panic!("fiasco input"),
                };
            }
            current_chunk = ProgramChunk {
                bitmask_zeroes,
                bitmask_ones,
                original_bitmask: caps[1].chars().map(|ch| ch as u8).collect(),
                instructions: Vec::new(),
            }
        } else if let Some(caps) = ASSIGNMENT_RE.captures(&line) {
            let address = caps[1].parse::<u64>().unwrap();
            let value = caps[2].parse::<u64>().unwrap();
            current_chunk.instructions.push((address, value));
        } else {
        }
    }
    result.push(current_chunk);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_example1() {
        let input = read_input(BufReader::new(
            File::open("inputs/day14/example.txt").unwrap(),
        ));
        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn test_example2() {
        let input = read_input(BufReader::new(
            File::open("inputs/day14/example2.txt").unwrap(),
        ));
        assert_eq!(part2(&input), 208);
    }

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day14/1.txt").unwrap()));
        assert_eq!(part1(&input), 5055782549997);
        assert_eq!(part2(&input), 4795970362286);
    }
}
