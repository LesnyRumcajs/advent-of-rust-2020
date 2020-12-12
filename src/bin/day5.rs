use std::{
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;
use simple_error::SimpleError;

struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn generate_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut row, mut column) = (0, 0);
        for ch in s.chars() {
            match ch {
                'F' => {
                    row <<= 1;
                }
                'B' => {
                    row <<= 1;
                    row |= 1;
                }
                'L' => {
                    column <<= 1;
                }
                'R' => {
                    column <<= 1;
                    column |= 1;
                }
                _ => panic!("fiasco!"),
            }
        }

        Ok(Seat { row, column })
    }
}

fn main() {
    let seats = read_seats(io::stdin().lock());
    println!("Day 5, part 1: {}", part1(&seats));
    println!("Day 5, part 2: {}", part2(&seats));
}

fn part1(seats: &[Seat]) -> usize {
    seats.iter().map(|seat| seat.generate_id()).max().unwrap()
}

fn part2(seats: &[Seat]) -> usize {
    seats
        .iter()
        .map(|seat| seat.generate_id())
        .sorted()
        .tuple_windows()
        .find(|(x, y)| *x != y - 1)
        .unwrap()
        .0
        + 1
}

fn read_seats<R: BufRead>(reader: R) -> Vec<Seat> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| Seat::from_str(&l).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let seats = read_seats(BufReader::new(File::open("inputs/day5/1.txt").unwrap()));
        assert_eq!(part1(&seats), 955);
        assert_eq!(part2(&seats), 569);
    }
}
