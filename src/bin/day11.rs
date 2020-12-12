use std::io::BufRead;

#[derive(Clone, PartialEq)]
struct Layout {
    layout: Vec<Vec<char>>,
}

impl Layout {
    fn from_lines(layout: Vec<Vec<char>>) -> Self {
        Layout { layout }
    }

    fn count_adjacent_occupied(&self, row: usize, col: usize) -> usize {
        let mut result = 0;
        let add_occupied = |row: i32, col: i32| {
            if row < 0
                || row >= self.layout.len() as i32
                || col < 0
                || col >= self.layout[0].len() as i32
            {
                0
            } else {
                (self.layout[row as usize][col as usize] == '#') as usize
            }
        };

        result += add_occupied(row as i32 - 1, col as i32 - 1);
        result += add_occupied(row as i32, col as i32 - 1);
        result += add_occupied(row as i32 + 1, col as i32 - 1);
        result += add_occupied(row as i32 - 1, col as i32);
        result += add_occupied(row as i32 + 1, col as i32);
        result += add_occupied(row as i32 - 1, col as i32 + 1);
        result += add_occupied(row as i32, col as i32 + 1);
        result + add_occupied(row as i32 + 1, col as i32 + 1)
    }

    fn add_occupied(&self, row: i32, row_mod: i32, col: i32, col_mod: i32) -> usize {
        let curr_row = row + row_mod;
        let curr_col = col + col_mod;
        if curr_row < 0
            || curr_row >= self.layout.len() as i32
            || curr_col < 0
            || curr_col >= self.layout[0].len() as i32
        {
            0
        } else {
            let ch = self.layout[(curr_row) as usize][(curr_col) as usize];
            if ch == '#' {
                1
            } else if ch == 'L' {
                0
            } else {
                self.add_occupied(curr_row, row_mod, curr_col, col_mod)
            }
        }
    }

    fn count_visible_occupied(&self, row: usize, col: usize) -> usize {
        let mut result = 0;
        result += self.add_occupied(row as i32, -1, col as i32, -1);
        result += self.add_occupied(row as i32, 0, col as i32, -1);
        result += self.add_occupied(row as i32, 1, col as i32, -1);
        result += self.add_occupied(row as i32, -1, col as i32, 0);
        result += self.add_occupied(row as i32, 1, col as i32, 0);
        result += self.add_occupied(row as i32, -1, col as i32, 1);
        result += self.add_occupied(row as i32, 0, col as i32, 1);
        result + self.add_occupied(row as i32, 1, col as i32, 1)
    }

    fn count_occupied(&self) -> u32 {
        self.layout.iter().fold(0, |acc, v| {
            acc + v.iter().fold(0, |acc, ch| acc + (*ch == '#') as u32)
        })
    }

    fn create_next_with_adjacent(&self) -> Option<Self> {
        let mut modified = false;
        let mut new_layout = Vec::new();
        for (x, row) in self.layout.iter().enumerate() {
            let mut line = Vec::new();
            for (y, val) in row.iter().enumerate() {
                let new_val = match val {
                    'L' => {
                        if self.count_adjacent_occupied(x, y) == 0 {
                            modified = true;
                            '#'
                        } else {
                            'L'
                        }
                    }
                    '#' => {
                        if self.count_adjacent_occupied(x, y) >= 4 {
                            modified = true;
                            'L'
                        } else {
                            '#'
                        }
                    }
                    _ => *val,
                };
                line.push(new_val);
            }
            new_layout.push(line);
        }
        if modified {
            Some(Layout::from_lines(new_layout))
        } else {
            None
        }
    }

    fn create_next_with_visible(&self) -> Option<Self> {
        let mut modified = false;
        let mut new_layout = Vec::new();
        for (x, row) in self.layout.iter().enumerate() {
            let mut line = Vec::new();
            for (y, val) in row.iter().enumerate() {
                let new_val = match val {
                    'L' => {
                        if self.count_visible_occupied(x, y) == 0 {
                            modified = true;
                            '#'
                        } else {
                            'L'
                        }
                    }
                    '#' => {
                        if self.count_visible_occupied(x, y) >= 5 {
                            modified = true;
                            'L'
                        } else {
                            '#'
                        }
                    }
                    _ => *val,
                };
                line.push(new_val);
            }
            new_layout.push(line);
        }
        if modified {
            Some(Layout::from_lines(new_layout))
        } else {
            None
        }
    }
}

fn main() {
    let layout = read_layout(std::io::stdin().lock());
    println!("Day 11, part 1: {}", part1(&layout));
    println!("Day 11, part 2: {}", part2(&layout));
}

fn part1(layout: &Layout) -> u32 {
    let mut current_layout = layout.clone();

    while let Some(new_layout) = current_layout.create_next_with_adjacent() {
        current_layout = new_layout;
    }

    current_layout.count_occupied()
}

fn part2(layout: &Layout) -> u32 {
    let mut current_layout = layout.clone();

    while let Some(new_layout) = current_layout.create_next_with_visible() {
        current_layout = new_layout;
    }

    current_layout.count_occupied()
}

fn read_layout<R: BufRead>(reader: R) -> Layout {
    Layout::from_lines(
        reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let layout = read_layout(BufReader::new(File::open("inputs/day11/1.txt").unwrap()));
        assert_eq!(part1(&layout), 2281);
        assert_eq!(part2(&layout), 2085);
    }
}
