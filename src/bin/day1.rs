use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let numbers = read_numbers();
    println!("Day 1, part 1: {}", part1(numbers.clone()).unwrap());
    println!("Day 1, part 2: {}", part2(numbers).unwrap());
}

fn part1(mut numbers: Vec<i32>) -> Option<i32> {
    numbers.sort_unstable();
    for number in numbers.iter() {
        let target = 2020 - number;
        if numbers.binary_search(&target).is_ok() {
            return Some(number * target);
        }
    }

    None
}

fn part2(mut numbers: Vec<i32>) -> Option<i32> {
    numbers.sort_unstable();
    for (first, second) in numbers.iter().combinations(2).map(|v| (v[0], v[1])) {
        let target = 2020 - first - second;
        if numbers.binary_search(&target).is_ok() {
            return Some(first * second * target);
        }
    }

    None
}

fn read_numbers() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|i| i.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(numbers).unwrap(), 514579)
    }

    #[test]
    fn part2_sample() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(numbers).unwrap(), 241861950)
    }
}
