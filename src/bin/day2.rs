use std::{
    io::{self, BufRead},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

struct PolicyPassword {
    first: usize,
    second: usize,
    character: char,
    pass: String,
}

impl FromStr for PolicyPassword {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+)\s+(\w):\s+(\w+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        Ok(PolicyPassword {
            first: caps[1].parse()?,
            second: caps[2].parse()?,
            character: caps[3].chars().next().unwrap(),
            pass: caps[4].to_owned(),
        })
    }
}

fn main() {
    let policies_passwords = read_policies_and_passwords(io::stdin().lock());
    println!("Day 2, part 2: {}", part1(&policies_passwords));
    println!("Day 2, part 2: {}", part2(&policies_passwords));
}

fn part1(policies_passwords: &[PolicyPassword]) -> usize {
    policies_passwords
        .iter()
        .filter(|l| {
            let occurences = l.pass.matches(l.character).count();
            occurences >= l.first && occurences <= l.second
        })
        .count()
}

fn part2(policies_passwords: &[PolicyPassword]) -> usize {
    policies_passwords
        .iter()
        .filter(|l| {
            (l.pass.chars().nth(l.first - 1).unwrap() == l.character)
                ^ (l.pass.chars().nth(l.second - 1).unwrap() == l.character)
        })
        .count()
}

fn read_policies_and_passwords<R: BufRead>(reader: R) -> Vec<PolicyPassword> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| PolicyPassword::from_str(&l).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input =
            read_policies_and_passwords(BufReader::new(File::open("inputs/day2/1.txt").unwrap()));
        assert_eq!(part1(&input), 640);
        assert_eq!(part2(&input), 472);
    }
}
