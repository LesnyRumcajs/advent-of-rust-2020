use std::io::{self, BufRead};

fn main() {
    let map = load_map(io::stdin().lock());
    println!("Day 3, part 1: {}", part1(&map));
    println!("Day 3, part 2: {}", part2(&map));
}

fn part1(map: &[String]) -> i64 {
    traverse(map, 3, 1)
}

fn part2(map: &[String]) -> i64 {
    traverse(map, 1, 1)
        * traverse(map, 3, 1)
        * traverse(map, 5, 1)
        * traverse(map, 7, 1)
        * traverse(map, 1, 2)
}

fn traverse(map: &[String], horizontal_slope: usize, vertical_slope: usize) -> i64 {
    let (mut x, mut y, mut trees) = (0, 0, 0);
    let map_width = map[0].len();
    while y < map.len() - vertical_slope {
        y += vertical_slope;
        x = (x + horizontal_slope) % map_width;
        if map[y].chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
    }

    trees
}

fn load_map<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().filter_map(Result::ok).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let map = load_map(BufReader::new(File::open("inputs/day3/1.txt").unwrap()));
        assert_eq!(part1(&map), 145);
        assert_eq!(part2(&map), 3424528800);
    }
}
