use std::{io::BufRead, str::FromStr};

use simple_error::SimpleError;

type Instructions = Vec<Instruction>;

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn axis_rotate(&self, rad: f64) -> Self {
        let sin = rad.sin() as i32;
        let cos = rad.cos() as i32;
        Point {
            x: self.x * cos + self.y * sin,
            y: -self.x * sin + self.y * cos,
        }
    }
}

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    TurnLeft(i32),
    TurnRight(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = s.chars().nth(0).unwrap();
        let val: i32 = s.chars().skip(1).collect::<String>().parse().unwrap();

        Ok(match action {
            'N' => Instruction::North(val),
            'S' => Instruction::South(val),
            'E' => Instruction::East(val),
            'W' => Instruction::West(val),
            'L' => Instruction::TurnLeft(val),
            'R' => Instruction::TurnRight(val),
            'F' => Instruction::Forward(val),
            _ => panic!("Unknown instruction!"),
        })
    }
}

fn main() {
    let instructions = read_instructions();
    println!("Day 12, part 1: {}", part1(&instructions));
    println!("Day 12, part 2: {}", part2(&instructions));
}

fn part1(instructions: &Instructions) -> i32 {
    let mut ship = Point::default();
    let mut direction = 0f64;
    for instruction in instructions.iter() {
        match instruction {
            Instruction::North(val) => {
                ship.y += val;
            }
            Instruction::South(val) => {
                ship.y -= val;
            }
            Instruction::East(val) => {
                ship.x += val;
            }
            Instruction::West(val) => {
                ship.x -= val;
            }
            Instruction::TurnLeft(val) => {
                direction += f64::from(*val).to_radians();
            }
            Instruction::TurnRight(val) => {
                direction -= f64::from(*val).to_radians();
            }
            Instruction::Forward(val) => {
                ship.x += (direction.cos() as i32) * val;
                ship.y += (direction.sin() as i32) * val;
            }
        }
    }
    ship.x.abs() + ship.y.abs()
}

fn part2(instructions: &Instructions) -> i32 {
    let mut ship = Point::default();
    let mut waypoint = Point { x: 10, y: 1 };
    for instruction in instructions.iter() {
        match instruction {
            Instruction::North(val) => {
                waypoint.y += val;
            }
            Instruction::South(val) => {
                waypoint.y -= val;
            }
            Instruction::East(val) => {
                waypoint.x += val;
            }
            Instruction::West(val) => {
                waypoint.x -= val;
            }
            Instruction::TurnLeft(val) => {
                waypoint = waypoint.axis_rotate(-f64::from(*val).to_radians());
            }
            Instruction::TurnRight(val) => {
                waypoint = waypoint.axis_rotate(f64::from(*val).to_radians());
            }
            Instruction::Forward(val) => {
                ship.x += waypoint.x * val;
                ship.y += waypoint.y * val;
            }
        }
    }
    ship.x.abs() + ship.y.abs()
}

fn read_instructions() -> Instructions {
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| Instruction::from_str(&l).ok())
        .collect()
}
