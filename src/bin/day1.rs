use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let numbers = read_numbers(io::stdin().lock());
    println!("Day 1, part 1: {}", part1(numbers.clone()));
    println!("Day 1, part 2: {}", part2(numbers));
}

fn part1(mut numbers: Vec<i32>) -> i32 {
    numbers.sort_unstable();
    for number in numbers.iter() {
        let target = 2020 - number;
        if numbers.binary_search(&target).is_ok() {
            return number * target;
        }
    }

    panic!("No solution found!");
}

fn part2(mut numbers: Vec<i32>) -> i32 {
    numbers.sort_unstable();
    for (first, second) in numbers.iter().combinations(2).map(|v| (v[0], v[1])) {
        let target = 2020 - first - second;
        if numbers.binary_search(&target).is_ok() {
            return first * second * target;
        }
    }

    panic!("No solution found!");
}

fn read_numbers<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|i| i.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(numbers), 514579)
    }

    #[test]
    fn part2_sample() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(numbers), 241861950)
    }

    #[test]
    fn test_solution() {
        let numbers = read_numbers(BufReader::new(File::open("inputs/day1/1.txt").unwrap()));
        assert_eq!(part1(numbers.clone()), 878724);
        assert_eq!(part2(numbers), 201251610);
    }
}
