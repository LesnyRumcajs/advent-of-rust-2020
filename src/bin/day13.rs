use std::io::BufRead;

struct Notes {
    earliest: i64,
    buses: Vec<(i64, i64)>,
}

/// Calculates the extended GCD
/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
/// Returns tuple (GCD, x, y)
/// Satisfying the ax + by = gcd(a,b)
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        (gcd, y1 - (b / a) * x1, x1)
    }
}

/// Calculates the modular multiplicative inverse
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
fn mod_inv(a: i64, m: i64) -> i64 {
    let (g, x, _y) = extended_gcd(a, m);
    assert!(g == 1, "modular inverse does not exist!");

    x % m
}

/// Calculates the Chinese Remainder
/// https://en.wikipedia.org/wiki/Chinese_remainder_theorem
fn calculate_ctr(inputs: &[(i64, i64)]) -> i64 {
    let product = inputs.iter().fold(1, |product, (_, pi)| product * pi);

    inputs.iter().fold(0, |sum, (residue, modulus)| {
        let p = product / modulus;
        (sum + residue * p * mod_inv(p, *modulus)) % product
    })
}

fn main() {
    let notes = read_notes(std::io::stdin().lock());
    println!("Day 13, part 1: {}", part1(&notes));
    println!("Day 13, part 2: {}", part2(&notes));
}

fn part1(notes: &Notes) -> i64 {
    let result = notes
        .buses
        .iter()
        .map(|(_, bus)| (bus, (notes.earliest / bus + 1) * bus - notes.earliest))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    result.0 * result.1
}

fn part2(notes: &Notes) -> i64 {
    calculate_ctr(&notes.buses)
}

fn read_notes<R: BufRead>(reader: R) -> Notes {
    let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    let earliest = lines[0].parse().unwrap();
    let buses = lines[1]
        .split(",")
        .enumerate()
        .filter(|(_, ch)| *ch != "x")
        .map(|(i, bus)| (-(i as i64), bus.parse().unwrap()))
        .collect();

    Notes { earliest, buses }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let notes = read_notes(BufReader::new(File::open("inputs/day13/1.txt").unwrap()));
        assert_eq!(part1(&notes), 333);
        assert_eq!(part2(&notes), 690123192779524);
    }
}
