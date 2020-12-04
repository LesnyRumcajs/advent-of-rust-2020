use std::{
    io::{self, BufRead},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{require_with, SimpleError};

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<u32>,
}

impl Passport {
    fn has_all_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_colour.is_some()
            && self.eye_colour.is_some()
            && self.passport_id.is_some()
    }

    fn validate(&self) -> Result<(), SimpleError> {
        if require_with!(self.birth_year, "no byr") < 1920
            || require_with!(self.birth_year, "no byr") > 2002
        {
            return Err(SimpleError::new("Invalid byr"));
        }

        if require_with!(self.issue_year, "no iyr") < 2010
            || require_with!(self.issue_year, "no iyr") > 2020
        {
            return Err(SimpleError::new("Invalid iyr"));
        }

        if require_with!(self.expiration_year, "no eyr") < 2020
            || require_with!(self.expiration_year, "no eyr") > 2030
        {
            return Err(SimpleError::new("Invalid eyr"));
        }

        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }
        let height_cap = require_with!(
            HEIGHT_RE.captures(require_with!(self.height.as_deref(), "no height")),
            "fiasco height"
        );
        let height_value = height_cap[1]
            .parse::<u32>()
            .map_err(|_| SimpleError::new("invalid height value"))?;
        match &height_cap[2] {
            "cm" => {
                if height_value < 150 || height_value > 193 {
                    return Err(SimpleError::new("Invalid height"));
                }
            }
            "in" => {
                if height_value < 59 || height_value > 76 {
                    return Err(SimpleError::new("Invalid height"));
                }
            }
            _ => return Err(SimpleError::new("Invalid height unit")),
        };

        lazy_static! {
            static ref HAIR_COLOUR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        }
        if !HAIR_COLOUR_RE.is_match(require_with!(self.hair_colour.as_deref(), "no hair colour")) {
            return Err(SimpleError::new("Invalid hair colour"));
        }

        lazy_static! {
            static ref EYE_COLOUR_RE: Regex =
                Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }
        if !EYE_COLOUR_RE.is_match(require_with!(self.eye_colour.as_deref(), "no eye colour")) {
            return Err(SimpleError::new("Invalid eye colour"));
        }

        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        if !PID_RE.is_match(require_with!(self.passport_id.as_deref(), "no passport id")) {
            return Err(SimpleError::new("Invalid passport id"));
        }

        Ok(())
    }
}

impl FromStr for Passport {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+):(\S+)").unwrap();
        }

        let mut passport = Passport::default();
        RE.captures_iter(s).for_each(|entry| {
            let key = entry.get(1).unwrap().as_str();
            let value = entry.get(2).unwrap().as_str();
            match key {
                "byr" => passport.birth_year = Some(value.parse().unwrap()),
                "iyr" => passport.issue_year = Some(value.parse().unwrap()),
                "eyr" => passport.expiration_year = Some(value.parse().unwrap()),
                "hgt" => passport.height = Some(value.to_owned()),
                "hcl" => passport.hair_colour = Some(value.to_owned()),
                "ecl" => passport.eye_colour = Some(value.to_owned()),
                "pid" => passport.passport_id = Some(value.to_owned()),
                "cid" => passport.country_id = Some(value.parse().unwrap()),
                _ => panic!("fiasco!"),
            }
        });

        Ok(passport)
    }
}

fn main() {
    let passports = read_passports();
    println!("Day 4, part 1: {}", part1(&passports));
    println!("Day 4, part 2: {}", part2(&passports));
}

fn part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.has_all_required_fields())
        .count()
}
fn part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.validate().is_ok())
        .count()
}

fn read_passports() -> Vec<Passport> {
    let mut result = Vec::new();
    let input: Vec<_> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    let mut buffer = String::new();
    for (num, line) in input.iter().enumerate() {
        if line.is_empty() && !buffer.is_empty() {
            result.push(Passport::from_str(&buffer).unwrap());
            buffer.clear();
        } else if num == input.len() - 1 {
            result.push(Passport::from_str(line).unwrap());
        } else {
            buffer += &format!(" {}", line);
        }
    }

    result
}
