use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
struct Rule {
    name: String,
    first: (u32, u32),
    second: (u32, u32),
}

#[derive(Clone, Debug)]
struct Input {
    rules: Vec<Rule>,
    ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Input {
    fn discard_invalid(&mut self) -> u32 {
        let mut result = 0;
        self.nearby_tickets = self
            .nearby_tickets
            .iter()
            .filter(|&ticket| {
                let mut ticket_valid = true;
                for num in ticket {
                    let mut valid = false;
                    for rule in self.rules.iter() {
                        if *num >= rule.first.0 && *num <= rule.first.1
                            || *num >= rule.second.0 && *num <= rule.second.1
                        {
                            valid = true;
                            break;
                        }
                    }

                    if !valid {
                        ticket_valid = false;
                        result += num;
                    }
                }
                ticket_valid
            })
            .cloned()
            .collect();
        result
    }
}

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 16, part 1: {}", part1(&input));
    println!("Day 16, part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u32 {
    input.clone().discard_invalid()
}
fn part2(input: &Input) -> u64 {
    let mut input = input.clone();
    input.discard_invalid();

    let mut all_tickets = input.nearby_tickets.clone();
    all_tickets.push(input.ticket.clone());

    let transposed: Vec<Vec<_>> = (0..all_tickets[0].len())
        .map(|i| {
            all_tickets
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut valid_positions: HashMap<String, Vec<u32>> = HashMap::new();
    for (pos, col) in transposed.iter().enumerate() {
        for rule in input.rules.iter() {
            if col.iter().all(|num| {
                *num >= rule.first.0 && *num <= rule.first.1
                    || *num >= rule.second.0 && *num <= rule.second.1
            }) {
                let entry = valid_positions
                    .entry(rule.name.clone())
                    .or_insert(Vec::new());
                entry.push(pos as u32);
            }
        }
    }

    while valid_positions.values().any(|v| v.len() != 1) {
        let set_positions: Vec<u32> = valid_positions
            .values()
            .filter(|v| v.len() == 1)
            .map(|v| v[0])
            .collect();

        for (_, v) in valid_positions.iter_mut().filter(|(_, v)| v.len() != 1) {
            v.retain(|v| !set_positions.contains(v));
        }
    }

    valid_positions
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| input.ticket[v[0] as usize] as u64)
        .product()
}

fn read_input<R: BufRead>(reader: R) -> Input {
    let mut rules: Vec<Rule> = Vec::new();
    let mut ticket: Vec<u32> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();

    let mut section = 0;
    for line in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
    {
        if line == "your ticket:" {
            section = 1;
            continue;
        }

        if line == "nearby tickets:" {
            section = 2;
            continue;
        }

        if section == 0 {
            lazy_static! {
                static ref RULE_RE: Regex =
                    Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
            }

            if let Some(caps) = RULE_RE.captures(&line) {
                rules.push(Rule {
                    name: caps[1].to_owned(),
                    first: (caps[2].parse().unwrap(), caps[3].parse().unwrap()),
                    second: (caps[4].parse().unwrap(), caps[5].parse().unwrap()),
                })
            }
        } else if section == 1 {
            ticket = line.split(',').map(|v| v.parse().unwrap()).collect();
        } else if section == 2 {
            nearby_tickets.push(line.split(',').map(|v| v.parse().unwrap()).collect());
        } else {
            panic!("fiasco section!");
        }
    }

    Input {
        rules,
        ticket,
        nearby_tickets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day16/1.txt").unwrap()));
        assert_eq!(part1(&input), 28873);
        assert_eq!(part2(&input), 2587271823407);
    }
}
