use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let numbers = read_numbers(io::stdin().lock());
    println!("Day 15, part 1: {}", part1(&numbers));
    println!("Day 15, part 2: {}", part2(&numbers));
}

fn part1(numbers: &[u32]) -> u32 {
    find_nth_result(numbers, 2020)
}

fn part2(numbers: &[u32]) -> u32 {
    find_nth_result(numbers, 30000000)
}

fn find_nth_result(numbers: &[u32], end_turn: u32) -> u32 {
    let mut num_occurences: HashMap<u32, u32> = HashMap::new();
    for (i, num) in numbers.iter().enumerate() {
        num_occurences.insert(*num, i as u32 + 1);
    }

    let mut last_num = 0;
    let mut turn = numbers.len() as u32 + 1;

    while turn != end_turn {
        if let Some(last_occurence) = num_occurences.get_mut(&last_num) {
            last_num = turn - *last_occurence;
            *last_occurence = turn;
        } else {
            num_occurences.insert(last_num, turn);
            last_num = 0;
        }

        turn += 1;
    }
    last_num
}

fn read_numbers<R: BufRead>(reader: R) -> Vec<u32> {
    reader.lines().filter_map(Result::ok).collect::<Vec<_>>()[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let numbers = read_numbers(BufReader::new(File::open("inputs/day15/1.txt").unwrap()));
        assert_eq!(part1(&numbers), 959);
        assert_eq!(part2(&numbers), 116590);
    }
}
