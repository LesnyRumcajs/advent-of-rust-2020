use std::io::{self, BufRead};

fn main() {
    let (deck1, deck2) = read_decks(io::stdin().lock());
    println!("Day 22, part 1: {}", part1(deck1.clone(), deck2.clone()));
    println!("Day 22, part 2: {}", part2(deck1.clone(), deck2.clone()));
}

fn part1(mut deck1: Vec<u32>, mut deck2: Vec<u32>) -> u32 {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = *deck1.first().unwrap();
        let card2 = *deck2.first().unwrap();

        deck1.remove(0);
        deck2.remove(0);
        if card1 > card2 {
            deck1.push(card1);
            deck1.push(card2);
        } else if card2 > card1 {
            deck2.push(card2);
            deck2.push(card1);
        } else {
            panic!("a draw!");
        }
    }
    if deck1.is_empty() { deck2 } else { deck1 }
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, v)| sum + (i as u32 + 1) * v)
}

fn play_game(mut deck1: Vec<u32>, mut deck2: Vec<u32>) -> (u32, Vec<u32>) {
    let mut cache: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let current = (deck1.clone(), deck2.clone());
        if cache.contains(&current) {
            return (1, deck1.clone());
        }

        cache.push(current);

        let card1 = *deck1.first().unwrap();
        let card2 = *deck2.first().unwrap();

        deck1.remove(0);
        deck2.remove(0);

        let result = if card1 <= deck1.len() as u32 && card2 <= deck2.len() as u32 {
            play_game(
                deck1.iter().take(card1 as usize).copied().collect(),
                deck2.iter().take(card2 as usize).copied().collect(),
            )
            .0
        } else {
            if card1 > card2 {
                1
            } else {
                2
            }
        };

        if result == 1 {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }

    if deck1.is_empty() {
        return (2, deck2);
    } else if deck2.is_empty() {
        return (1, deck1);
    } else {
        panic!("fiasco");
    }
}

fn part2(deck1: Vec<u32>, deck2: Vec<u32>) -> u32 {
    play_game(deck1.clone(), deck2.clone())
        .1
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, v)| sum + (i as u32 + 1) * v)
}

fn read_decks<R: BufRead>(reader: R) -> (Vec<u32>, Vec<u32>) {
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();

    let mut section = 0;
    for line in reader.lines().filter_map(Result::ok) {
        if line.is_empty() {
            section += 1;
            continue;
        }

        if line.starts_with("P") {
            continue;
        }

        if section == 0 {
            deck1.push(line.parse().unwrap());
        } else {
            deck2.push(line.parse().unwrap());
        }
    }

    (deck1, deck2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let (deck1, deck2) = read_decks(BufReader::new(File::open("inputs/day22/1.txt").unwrap()));
        assert_eq!(part1(deck1.clone(), deck2.clone()), 30138);
        assert_eq!(part2(deck1.clone(), deck2.clone()), 31587);
    }
}
