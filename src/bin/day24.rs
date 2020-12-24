use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Step {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn generate_adjacent(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 2, self.y),
            Point::new(self.x - 2, self.y),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y - 1),
        ]
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn main() {
    let steps = read_steps(io::stdin().lock());
    let flipped = part1(&steps);
    println!("Day 24, part 1: {}", flipped.len());
    println!("Day 24, part 2: {}", part2(flipped));
}

fn part1(steps: &[Vec<Step>]) -> HashSet<Point> {
    let mut flipped: HashSet<Point> = HashSet::new();
    for single_steps in steps.iter() {
        let (mut small_x, mut small_y) = (0, 0);
        for step in single_steps.iter() {
            match step {
                Step::E => small_x += 2,
                Step::SE => {
                    small_x += 1;
                    small_y += 1;
                }
                Step::SW => {
                    small_x -= 1;
                    small_y += 1;
                }
                Step::W => small_x -= 2,
                Step::NW => {
                    small_x -= 1;
                    small_y -= 1;
                }
                Step::NE => {
                    small_x += 1;
                    small_y -= 1;
                }
            }
        }

        let p = Point::new(small_x, small_y);
        if flipped.contains(&p) {
            flipped.remove(&p);
        } else {
            flipped.insert(p);
        }
    }
    flipped
}
fn part2(mut black: HashSet<Point>) -> usize {
    for _ in 0..100 {
        let mut new_black: HashSet<Point> = HashSet::new();
        for tile in black.iter() {
            let tile_adjacents = tile.generate_adjacent();
            let black_adjacent_count = tile_adjacents.iter().filter(|t| black.contains(t)).count();
            if black_adjacent_count > 0 && black_adjacent_count < 3 {
                new_black.insert(tile.clone());
            }

            for adjacent in tile_adjacents {
                if black.contains(&adjacent) {
                    continue;
                }

                let adjacent_count = adjacent
                    .generate_adjacent()
                    .iter()
                    .filter(|t| black.contains(t))
                    .count();
                if adjacent_count == 2 {
                    new_black.insert(adjacent);
                }
            }
        }

        black = new_black;
    }
    black.len()
}

fn read_steps<R: BufRead>(reader: R) -> Vec<Vec<Step>> {
    let mut result = Vec::new();
    for line in reader.lines().filter_map(Result::ok) {
        let mut steps: Vec<Step> = Vec::new();
        let mut last: Option<char> = None;
        for ch in line.chars() {
            if last.is_none() {
                match ch {
                    'e' => steps.push(Step::E),
                    'w' => steps.push(Step::W),
                    _ => last = Some(ch),
                };
            } else {
                let pre = last.unwrap();
                last = None;
                match ch {
                    'e' if pre == 's' => steps.push(Step::SE),
                    'e' if pre == 'n' => steps.push(Step::NE),
                    'w' if pre == 'n' => steps.push(Step::NW),
                    'w' if pre == 's' => steps.push(Step::SW),
                    _ => panic!("fiasco!"),
                }
            }
        }
        result.push(steps);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let steps = read_steps(BufReader::new(File::open("inputs/day24/1.txt").unwrap()));
        let flipped = part1(&steps);
        assert_eq!(flipped.len(), 375);
        assert_eq!(part2(flipped), 3937);
    }
}
