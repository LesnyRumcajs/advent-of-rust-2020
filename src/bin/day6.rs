use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use itertools::Itertools;

fn main() {
    let input: Vec<_> = io::stdin().lock().lines().filter_map(Result::ok).collect();
    println!("Day 6, part 1: {}", part1(&input));
    println!("Day 6, part 2: {}", part2(&input));
}

fn part1(input: &[String]) -> usize {
    let mut group: String = String::new();
    let mut result = 0;
    for (num, line) in input.iter().enumerate() {
        if line.is_empty() {
            result += group.chars().unique().count();
            group.clear();
        } else if num == input.len() - 1 {
            group += line;
            result += group.chars().unique().count();
            group.clear();
        } else {
            group += line;
        }
    }

    result
}
fn part2(input: &[String]) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    let mut result = 0;
    let mut is_new_group = true;

    for (num, line) in input.iter().enumerate() {
        if line.is_empty() {
            result += set.len();
            set.clear();
            is_new_group = true;
        } else if num == input.len() - 1 {
            result += if is_new_group {
                line.chars().collect::<HashSet<_>>().len()
            } else {
                set.intersection(&line.chars().collect())
                    .cloned()
                    .collect::<HashSet<_>>()
                    .len()
            };
        } else {
            set = if is_new_group {
                is_new_group = false;
                line.chars().collect()
            } else {
                set.intersection(&line.chars().collect()).cloned().collect()
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input: Vec<_> = BufReader::new(File::open("inputs/day6/1.txt").unwrap())
            .lines()
            .filter_map(Result::ok)
            .collect();
        assert_eq!(part1(&input), 6799);
        assert_eq!(part2(&input), 3354);
    }
}
