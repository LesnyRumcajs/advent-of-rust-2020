use std::io::{self, BufRead};

fn main() {
    let (card_pubkey, door_pubkey) = read_keys(io::stdin().lock());
    println!("Day 25, part 1: {}", part1(card_pubkey, door_pubkey));
}

fn calculate_loop_size(subject: u64, expected: u64) -> u64 {
    let mut value = 1;
    let mut loop_count = 0;

    while value != expected {
        loop_count += 1;
        value = (subject * value) % 20201227;
    }

    loop_count
}

fn calculate_encryption_key(subject: u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1, |value, _| (subject * value) % 20201227)
}

fn part1(card_pubkey: u64, door_pubkey: u64) -> u64 {
    calculate_encryption_key(door_pubkey, calculate_loop_size(7, card_pubkey))
}

fn read_keys<R: BufRead>(reader: R) -> (u64, u64) {
    let nums: Vec<_> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let (card_pubkey, door_pubkey) =
            read_keys(BufReader::new(File::open("inputs/day25/1.txt").unwrap()));
        assert_eq!(part1(card_pubkey, door_pubkey), 9620012);
    }
}
