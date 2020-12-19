use apply::Apply;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Clone)]
enum Rule {
    CHAR(char),
    SINGLE(Vec<u32>),
    OR((Vec<u32>, Vec<u32>)),
}

#[derive(Default, Clone)]
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
        Message::validate_chunk(&self.message, rules, 0).contains(&Some(""))
    }

    fn validate_chunk<'a>(chunk: &'a str, rules: &Rules, rule_id: u32) -> Vec<Option<&'a str>> {
        match rules.rules[&rule_id] {
            Rule::CHAR(c) if chunk.starts_with(c) => vec![Some(&chunk[1..])],
            Rule::CHAR(_) => vec![None],
            Rule::SINGLE(ref subrules) if subrules.first() == Some(&rule_id) => vec![None],
            Rule::SINGLE(ref subrules) => {
                subrules.iter().fold(vec![Some(chunk)], |m, &subrule_id| {
                    m.iter()
                        .flat_map(|chunk| match chunk {
                            Some(chunk) if !chunk.is_empty() => {
                                Message::validate_chunk(chunk, &rules, subrule_id)
                            }
                            _ => vec![None],
                        })
                        .collect()
                })
            }
            Rule::OR((ref subrules1, ref subrules2)) => [subrules1, subrules2]
                .iter()
                .flat_map(|subrules| {
                    subrules
                        .iter()
                        .fold(vec![Some(chunk)], |chunk, &subrule_id| {
                            chunk
                                .iter()
                                .flat_map(|chunk| match chunk {
                                    Some(chunk) if !chunk.is_empty() => {
                                        Message::validate_chunk(chunk, &rules, subrule_id)
                                    }
                                    _ => vec![None],
                                })
                                .collect()
                        })
                })
                .collect::<Vec<_>>()
                .apply(|result| match result {
                    result if result.is_empty() => vec![None],
                    result if result.contains(&Some("")) => vec![Some("")],
                    result => result,
                }),
        }
    }
}

fn main() {
    let (rules, messages) = read_input(io::stdin().lock());
    println!("Day 19, part 1: {}", part1(&rules, &messages));
    println!("Day 19, part 2: {}", part2(&rules, &messages));
}

fn part1(rules: &Rules, messages: &[Message]) -> usize {
    messages
        .iter()
        .filter(|message| message.validate(rules))
        .count()
}
fn part2(rules: &Rules, messages: &[Message]) -> usize {
    let mut modified_rules = rules.clone();

    if let Some(rule) = modified_rules.rules.get_mut(&8) {
        *rule = Rule::OR((vec![42], vec![42, 8]));
    }

    if let Some(rule) = modified_rules.rules.get_mut(&11) {
        *rule = Rule::OR((vec![42, 31], vec![42, 11, 31]));
    }

    messages
        .iter()
        .filter(|message| message.validate(&modified_rules))
        .count()
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
    fn test_solution() {
        let (rules, messages) =
            read_input(BufReader::new(File::open("inputs/day19/1.txt").unwrap()));
        assert_eq!(part1(&rules, &messages), 176);
        assert_eq!(part2(&rules, &messages), 352);
    }
}
