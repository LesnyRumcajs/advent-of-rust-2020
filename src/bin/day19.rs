use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

enum Rule {
    CHAR(char),
    SINGLE(Vec<u32>),
    OR((Vec<u32>, Vec<u32>)),
}

#[derive(Default)]
struct Rules {
    rules: HashMap<u32, Rule>,
}

impl Rules {
    fn add_from_str(&mut self, s: &str) {
        lazy_static! {
            static ref OR_RE: Regex = Regex::new(r"^(\d+): ([\w ]+)\|([\w ]+)$").unwrap();
            static ref SINGLE_RE: Regex = Regex::new(r"^(\d+): ([\w ]+)$").unwrap();
            static ref CHAR_RE: Regex = Regex::new("^(\\d+): \"(\\w)\"$").unwrap();
        }

        if let Some(caps) = OR_RE.captures(s) {
            let id: u32 = caps[1].parse().unwrap();
            let first: Vec<_> = caps[2]
                .split(' ')
                .map(|ch| ch.parse::<u32>())
                .filter_map(Result::ok)
                .collect();
            let second: Vec<_> = caps[3]
                .split(' ')
                .map(|ch| ch.parse::<u32>())
                .filter_map(Result::ok)
                .collect();

            self.rules.insert(id, Rule::OR((first, second)));
        } else if let Some(caps) = SINGLE_RE.captures(s) {
            let id: u32 = caps[1].parse().unwrap();
            let first: Vec<_> = caps[2]
                .split(' ')
                .map(|ch| ch.parse::<u32>())
                .filter_map(Result::ok)
                .collect();

            self.rules.insert(id, Rule::SINGLE(first));
        } else if let Some(caps) = CHAR_RE.captures(s) {
            let id: u32 = caps[1].parse().unwrap();
            let ch: char = caps[2].chars().next().unwrap();

            self.rules.insert(id, Rule::CHAR(ch));
        } else {
            panic!("bogus input!");
        }
    }
}

struct Message {
    message: String,
}

impl Message {
    fn validate(&self, rules: &Rules) -> bool {
        let mut rule = rules.rules.get(&0).unwrap();

        let (valid, count) = Message::validate_chunk(&self.message, rules, rule);

        valid && count == self.message.len()
    }

    fn validate_chunk_single(s: &str, rules: &Rules, subrules: &[u32]) -> (bool, usize) {
        let mut pos: usize = 0;
        for subrule_id in subrules.iter() {
            let subrule = rules.rules.get(subrule_id).unwrap();
            let (valid, count) = Message::validate_chunk(&s[pos..s.len()], rules, subrule);

            pos += count;
            if !valid {
                return (false, pos);
            }
        }
        return (true, pos);
    }

    fn validate_chunk(s: &str, rules: &Rules, rule: &Rule) -> (bool, usize) {
        if let Rule::CHAR(ch) = rule {
            if let Some(chunk_char) = s.chars().next() {
                return (chunk_char == *ch, 1);
            } else {
                return (false, 1);
            }
        } else if let Rule::SINGLE(subrules) = rule {
            return Message::validate_chunk_single(s, rules, subrules);
        } else if let Rule::OR((subrules1, subrules2)) = rule {
            let result1 = Message::validate_chunk_single(s, rules, subrules1);
            if result1.0 {
                return result1;
            }
            return Message::validate_chunk_single(s, rules, subrules2);
        }
        panic!("fiasco");
    }
}

fn main() {
    let (rules, messages) = read_input(io::stdin().lock());
    println!("Day 19, part 1: {}", part1(&rules, &messages));
    //println!("Day 19, part 2: {}", part2());
}

fn part1(rules: &Rules, messages: &[Message]) -> u32 {
    let mut valid_count = 0;

    for message in messages.iter() {
        if message.validate(rules) {
            valid_count += 1;
        }
    }

    valid_count
}
fn part2() -> i32 {
    unimplemented!();
}

fn read_input<R: BufRead>(reader: R) -> (Rules, Vec<Message>) {
    let mut section = 0;
    let mut rules = Rules::default();
    let mut messages = Vec::new();

    for line in reader.lines().filter_map(Result::ok) {
        if section == 0 {
            if line.is_empty() {
                section += 1;
                continue;
            }
            rules.add_from_str(&line);
        } else {
            messages.push(Message { message: line });
        }
    }

    (rules, messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {}
}
