use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let game = read_game(io::stdin().lock());
    println!("Day 17, part 1: {}", part1(&game));
    println!("Day 17, part 2: {}", part2(&game));
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

type Space = HashMap<Point, bool>;

#[derive(Clone, Debug)]
struct Game {
    space: HashMap<Point, bool>,
}

impl Game {
    fn tick3d(&mut self) {
        self.tick(0)
    }

    fn tick4d(&mut self) {
        self.tick(1)
    }

    fn tick(&mut self, fourth_search: i32) {
        let mut new_space = Space::new();

        let count_neighbours = |point: &Point| {
            let mut active_neighbours = 0;
            for w in point.w - fourth_search..=point.w + fourth_search {
                for z in point.z - 1..=point.z + 1 {
                    for y in point.y - 1..=point.y + 1 {
                        for x in point.x - 1..=point.x + 1 {
                            let this_point = Point { x, y, z, w };
                            if &this_point == point {
                                continue;
                            }
                            if let Some(neighbour) = self.space.get(&Point { x, y, z, w }) {
                                if *neighbour {
                                    active_neighbours += 1;
                                }
                            }
                        }
                    }
                }
            }
            active_neighbours
        };

        for (point, is_active) in self.space.iter() {
            let mut active_neighbours = 0;
            for w in point.w - fourth_search..=point.w + fourth_search {
                for z in point.z - 1..=point.z + 1 {
                    for y in point.y - 1..=point.y + 1 {
                        for x in point.x - 1..=point.x + 1 {
                            let this_point = Point { x, y, z, w };
                            if &this_point == point {
                                continue;
                            }
                            if let Some(neighbour) = self.space.get(&this_point) {
                                if *neighbour {
                                    active_neighbours += 1;
                                }
                            } else {
                                let neighbours = count_neighbours(&this_point);
                                if neighbours == 3 {
                                    new_space.insert(this_point, true);
                                }
                            }
                        }
                    }
                }
            }

            if *is_active {
                new_space.insert(
                    point.clone(),
                    active_neighbours == 2 || active_neighbours == 3,
                );
            } else {
                new_space.insert(point.clone(), active_neighbours == 3);
            }
        }

        self.space = new_space;
    }

    fn count_active(&self) -> usize {
        let mut result = 0;
        for (_, v) in self.space.iter() {
            if *v {
                result += 1;
            }
        }

        result
    }
}

fn part1(game: &Game) -> usize {
    let mut game = game.clone();
    for _ in 0..6 {
        game.tick3d();
    }

    game.count_active()
}

fn part2(game: &Game) -> usize {
    let mut game = game.clone();
    for _ in 0..6 {
        game.tick4d();
    }

    game.count_active()
}

fn read_game<R: BufRead>(reader: R) -> Game {
    let mut space = Space::new();
    for (y, line) in reader.lines().filter_map(Result::ok).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            space.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                },
                ch == '#',
            );
        }
    }

    Game { space }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let game = read_game(BufReader::new(File::open("inputs/day17/1.txt").unwrap()));
        assert_eq!(part1(&game), 401);
        assert_eq!(part2(&game), 2224);
    }
}
