use std::io::{self, BufRead};

struct Equation {
    raw: String,
}

impl Equation {
    fn calculate(&self, plus_precedence: bool) -> i64 {
        // Shunting-Yard algorithm
        // https://en.wikipedia.org/wiki/Shunting-yard_algorithm
        let mut output = Vec::new();
        let mut operators = Vec::new();
        for ch in self.raw.chars() {
            if ch.is_digit(10) {
                output.push(ch);
            } else if Equation::is_operator(ch) {
                if plus_precedence {
                    while let Some(operator) = operators.pop() {
                        if operator == '(' {
                            operators.push('(');
                            break;
                        } else {
                            output.push(operator);
                        }
                    }
                } else {
                    if let Some(operator) = operators.pop() {
                        if operator == '(' {
                            operators.push('(');
                        } else if operator == '+' {
                            output.push('+');
                        } else {
                            operators.push(operator);
                        }
                    }
                }
                operators.push(ch);
            } else if ch == '(' {
                operators.push(ch);
            } else if ch == ')' {
                while let Some(operator) = operators.pop() {
                    if operator == '(' {
                        break;
                    } else {
                        output.push(operator);
                    }
                }
            }
        }

        while let Some(operator) = operators.pop() {
            output.push(operator);
        }

        // Evaluation of Reverse Polish Notation
        let mut stack: Vec<i64> = Vec::new();
        for &token in output.iter() {
            match token.to_string().parse::<i64>() {
                Ok(num) => {
                    stack.push(num);
                }
                Err(_) => match token {
                    '+' => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a + b);
                    }
                    '*' => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a * b);
                    }
                    _ => panic!("Unknown operator!"),
                },
            }
        }
        stack.pop().unwrap_or(0)
    }

    fn is_operator(ch: char) -> bool {
        ch == '+' || ch == '*'
    }
}

fn main() {
    let equations = read_equations(io::stdin().lock());
    println!("Day 18, part 1: {}", part1(&equations));
    println!("Day 18, part 2: {}", part2(&equations));
}

fn part1(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .fold(0, |acc, eq| acc + eq.calculate(false))
}

fn part2(equations: &[Equation]) -> i64 {
    equations.iter().fold(0, |acc, eq| acc + eq.calculate(true))
}

fn read_equations<R: BufRead>(reader: R) -> Vec<Equation> {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| Equation {
            raw: l.replace(" ", ""),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let equations = read_equations(BufReader::new(File::open("inputs/day18/1.txt").unwrap()));
        assert_eq!(part1(&equations), 283582817678281);
        assert_eq!(part2(&equations), 4940631886147);
    }
}
