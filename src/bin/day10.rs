use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let adapters = read_adapters();
    println!("Day 10, part 1: {}", part1(&adapters));
    println!("Day 10, part 2: {}", part2(&adapters));
}

fn part1(adapters: &[u32]) -> u32 {
    let result = adapters
        .iter()
        .tuple_windows()
        .map(|(first, second)| second - first)
        .fold((0, 0), |sum, v| {
            if v == 1 {
                (sum.0 + 1, sum.1)
            } else {
                (sum.0, sum.1 + 1)
            }
        });
    result.0 * result.1
}
fn part2(adapters: &[u32]) -> u64 {
    let mut last_adapter = 0;
    let mut current_chunk: Vec<u32> = Vec::new();
    let mut result = 1;
    for adapter in adapters.iter() {
        if adapter - last_adapter == 3 || current_chunk.len() == 5 {
            if current_chunk.len() == 3 {
                result *= 2;
            } else if current_chunk.len() == 4 {
                result *= 4;
            } else if current_chunk.len() == 5 {
                result *= 7;
            }
            current_chunk.clear();
            current_chunk.push(*adapter);
        } else {
            current_chunk.push(*adapter);
        }
        last_adapter = *adapter;
    }
    result
}

fn read_adapters() -> Vec<u32> {
    let mut adapters: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();
    adapters
}
