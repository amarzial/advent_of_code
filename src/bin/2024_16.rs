use std::collections::HashSet;

use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;
use pathfinding::prelude::{astar_bag, dijkstra};

type Coord = Coordinate<i32>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    pos: Coord,
    direction: Coord,
}

fn load_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn _print_path(map: &Vec<Vec<char>>, path: &Vec<Pos>) {
    let mut map = map.clone();
    for p in path {
        map[p.pos.y as usize][p.pos.x as usize] = 'X';
    }
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part_one(input: &str) -> Option<usize> {
    let map = load_input(input);
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                'S' => start = Coord::new(j as i32, i as i32),
                'E' => end = Coord::new(j as i32, i as i32),
                _ => {}
            }
        }
    }

    let path: (Vec<Pos>, usize) = dijkstra(
        &Pos {
            pos: start,
            direction: Coord::new(1, 0),
        },
        |n| {
            let neigh = n
                .pos
                .neighbors()
                .iter()
                .filter(|node| {
                    node.x >= 0
                        && node.x < map[0].len() as i32
                        && node.y >= 0
                        && node.y < map.len() as i32
                        && (map[node.y as usize][node.x as usize] == '.'
                            || map[node.y as usize][node.x as usize] == 'E')
                        && **node != (n.pos - n.direction)
                })
                .map(|node| {
                    let dir = *node - n.pos;
                    (
                        Pos {
                            pos: *node,
                            direction: dir,
                        },
                        if dir == n.direction { 1 } else { 1001 },
                    )
                })
                .collect_vec();
            neigh
        },
        |n| n.pos == end,
    )
    .unwrap();

    Some(path.1)
}

fn part_two(input: &str) -> Option<usize> {
    let map = load_input(input);
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                'S' => start = Coord::new(j as i32, i as i32),
                'E' => end = Coord::new(j as i32, i as i32),
                _ => {}
            }
        }
    }

    let paths = astar_bag(
        &Pos {
            pos: start,
            direction: Coord::new(1, 0),
        },
        |n| {
            let neigh = n
                .pos
                .neighbors()
                .iter()
                .filter(|node| {
                    node.x >= 0
                        && node.x < map[0].len() as i32
                        && node.y >= 0
                        && node.y < map.len() as i32
                        && (map[node.y as usize][node.x as usize] == '.'
                            || map[node.y as usize][node.x as usize] == 'E')
                        && **node != (n.pos - n.direction)
                })
                .map(|node| {
                    let dir = *node - n.pos;
                    (
                        Pos {
                            pos: *node,
                            direction: dir,
                        },
                        if dir == n.direction { 1 } else { 1001 },
                    )
                })
                .collect_vec();
            neigh
        },
        |n| n.pos.manhattan(&end) as usize,
        |n| n.pos == end,
    )
    .unwrap();

    let mut all = HashSet::new();
    for path in paths.0 {
        all.extend(path.iter().map(|p| p.pos));
    }

    Some(all.len())
}
fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 16);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 16);
        assert_eq!(part_one(&input), Some(7036));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 16);
        assert_eq!(part_two(&input), Some(45));
    }
}
