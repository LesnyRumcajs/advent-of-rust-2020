use std::{
    collections::HashMap,
    io::{self, BufRead},
};

type TileData = Vec<Vec<u8>>;

#[derive(Debug)]
struct Tile {
    id: u32,
    data: TileData,
}

fn main() {
    let tiles = read_tiles(io::stdin().lock());
    println!("Day 20, part 1: {}", part1(&tiles));
    println!("Day 20, part 2: {}", part2());
}

fn part1(tiles: &[Tile]) -> u64 {
    let mut edges_by_ids: HashMap<u32, Vec<Vec<u8>>> = HashMap::new();
    for tile in tiles.iter() {
        let edge_upper = tile.data[0].clone();
        let edge_bottom = tile.data.last().unwrap().clone();
        let edge_left: Vec<_> = tile.data.iter().map(|row| *row.first().unwrap()).collect();
        let edge_right: Vec<_> = tile.data.iter().map(|row| *row.last().unwrap()).collect();

        edges_by_ids.insert(
            tile.id,
            vec![edge_upper, edge_bottom, edge_left, edge_right],
        );
    }

    let mut result = 1;
    for (tile_id, edges) in edges_by_ids.iter() {
        let mut valid_neighbours = 0;
        for edge in edges.iter() {
            for (neighbour_id, neighbour_edges) in edges_by_ids.iter() {
                if tile_id == neighbour_id {
                    continue;
                }

                for neighbour_edge in neighbour_edges.iter() {
                    if neighbour_edge == edge
                        || neighbour_edge == &edge.iter().rev().cloned().collect::<Vec<_>>()
                    {
                        valid_neighbours += 1;
                        break;
                    }
                }
            }
        }

        if valid_neighbours == 2 {
            result *= *tile_id as u64;
        }
    }

    result
}
fn part2() -> i32 {
    unimplemented!();
}

fn read_tiles<R: BufRead>(reader: R) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut current_id = 0;
    let mut current_data: TileData = Vec::new();
    for line in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
    {
        if line.starts_with("Tile") {
            if !current_data.is_empty() {
                tiles.push(Tile {
                    id: current_id,
                    data: current_data.clone(),
                });
                current_data.clear();
            }
            current_id = line
                .split(' ')
                .nth(1)
                .unwrap()
                .replace(":", "")
                .parse()
                .unwrap();
        } else {
            current_data.push(line.into_bytes());
        }
    }

    tiles.push(Tile {
        id: current_id,
        data: current_data,
    });

    tiles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let tiles = read_tiles(BufReader::new(File::open("inputs/day20/1.txt").unwrap()));
        assert_eq!(part1(&tiles), 19955159604613);
    }
}
