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
    let policies_passwords = read_policies_and_passwords();
    println!(
        "Day 2, part 2: {}",
        policies_passwords
            .iter()
            .filter(|l| {
                let occurences = l.pass.matches(l.character).count();
                occurences >= l.first && occurences <= l.second
            })
            .count()
    );

    println!(
        "Day 2, part 2: {}",
        policies_passwords
            .iter()
            .filter(
                |l| (l.pass.chars().nth(l.first - 1).unwrap() == l.character)
                    ^ (l.pass.chars().nth(l.second - 1).unwrap() == l.character)
            )
            .count()
    );
}

fn read_policies_and_passwords() -> Vec<PolicyPassword> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| PolicyPassword::from_str(&l).ok())
        .collect()
}
