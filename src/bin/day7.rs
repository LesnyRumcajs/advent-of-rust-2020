use std::{
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{require_with, SimpleError};

struct Rule {
    name: String,
    children: Vec<(String, usize)>,
}

impl FromStr for Rule {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARENT_RE: Regex = Regex::new(r"^(.*) bags contain (.*)$").unwrap();
            static ref CHILD_RE: Regex = Regex::new(r"((\d) (\w+ \w+) bags?)").unwrap();
        }
        let caps = require_with!(PARENT_RE.captures(s), "invalid rule!");
        let (name, rules) = (caps[1].to_owned(), caps[2].to_owned());
        let children = CHILD_RE
            .find_iter(&rules)
            .map(|rule| {
                let chunks = rule.as_str().split(' ').collect::<Vec<_>>();
                let num: usize = chunks[0].parse().unwrap();
                let name = format!("{} {}", chunks[1], chunks[2]);
                (name, num)
            })
            .collect();
        Ok(Rule { name, children })
    }
}

fn main() {
    let rules = read_rules(io::stdin().lock());
    println!("Day 7, part 1: {}", part1(&rules));
    println!("Day 7, part 2: {}", part2(&rules));
}

fn find_bag(rules: &[Rule], name: &str) -> Vec<String> {
    let mut results = Vec::new();
    for rule in rules.iter() {
        for children in rule.children.iter() {
            if children.0 == name {
                results.push(rule.name.to_owned());
                results.extend(find_bag(rules, &rule.name));
            }
        }
    }
    results
}
fn count_bags(rules: &[Rule], name: &str) -> usize {
    rules
        .iter()
        .find(|rule| rule.name == name)
        .unwrap()
        .children
        .iter()
        .fold(1, |sum, child| sum + child.1 * count_bags(rules, &child.0))
}

fn part1(rules: &[Rule]) -> usize {
    let bags = find_bag(rules, "shiny gold");
    bags.iter().unique().count()
}
fn part2(rules: &[Rule]) -> usize {
    count_bags(rules, "shiny gold") - 1
}

fn read_rules<R: BufRead>(reader: R) -> Vec<Rule> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| Rule::from_str(&l).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let rules = read_rules(BufReader::new(File::open("inputs/day7/1.txt").unwrap()));
        assert_eq!(part1(&rules), 316);
        assert_eq!(part2(&rules), 11310);
    }
}
