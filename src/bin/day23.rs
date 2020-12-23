use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn main() {
    let order = read_order(io::stdin().lock());
    println!("Day 23, part 1: {}", part1(&order));
    println!("Day 23, part 2: {}", part2(&order));
}

fn select_destination(current_cup: u32, cup1: u32, cup2: u32, cup3: u32) -> Option<u32> {
    let mut target = current_cup - 1;
    while target != 0 {
        if target == cup1 || target == cup2 || target == cup3 {
            target -= 1;
            continue;
        }

        return Some(target);
    }
    return None;
}

fn part1(order: &[u32]) -> i32 {
    let mut cups: VecDeque<u32> = order.iter().copied().collect();

    for _ in 0..100 {
        let current_cup = cups.pop_front().unwrap();
        let cup1 = cups.pop_front().unwrap();
        let cup2 = cups.pop_front().unwrap();
        let cup3 = cups.pop_front().unwrap();

        let destination = select_destination(current_cup, cup1, cup2, cup3)
            .unwrap_or(*cups.iter().max().unwrap());

        let index = cups.iter().position(|&v| v == destination).unwrap();
        cups.insert(index + 1, cup3);
        cups.insert(index + 1, cup2);
        cups.insert(index + 1, cup1);
        cups.push_back(current_cup);
    }
    while *cups.get(0).unwrap() != 1 {
        cups.rotate_left(1);
    }

    cups.iter()
        .skip(1)
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}
fn part2(order: &[u32]) -> u64 {
    let mut cups: VecDeque<u32> = order.iter().copied().collect();
    let mut max = *cups.iter().max().unwrap();
    while cups.len() < 1000000 {
        max += 1;
        cups.push_back(max);
    }

    for i in 0..10000000 {
        let current_cup = cups.pop_front().unwrap();
        let cup1 = cups.pop_front().unwrap();
        let cup2 = cups.pop_front().unwrap();
        let cup3 = cups.pop_front().unwrap();

        let destination =
            select_destination(current_cup, cup1, cup2, cup3).unwrap_or(cups.len() as u32);

        let index = cups.iter().position(|&v| v == destination).unwrap();
        cups.insert(index + 1, cup3);
        cups.insert(index + 1, cup2);
        cups.insert(index + 1, cup1);
        cups.push_back(current_cup);

        if i % 100000 == 0 {
            println!("{}%", i / 100000);
        }
    }
    let index = cups.iter().position(|&v| v == 1).unwrap();
    cups[index + 1] as u64 * cups[index + 2] as u64
}

fn read_order<R: BufRead>(reader: R) -> Vec<u32> {
    reader
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let cups = read_order(BufReader::new(File::open("inputs/day23/1.txt").unwrap()));
        assert_eq!(part1(&cups), 36472598);

        // too slow for CI, TODO optimize this
        //assert_eq!(part2(&cups), 90481418730);
    }
}
