use std::collections::HashSet;

use aoc::helpers::coordinate::Coordinate;
use pathfinding::prelude::bfs_reach;

type Garden = Vec<Vec<char>>;
type Coord = Coordinate<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Side {
    pos: Coord,
    facing: Coord,
}

fn read_input(input: &str) -> Garden {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_section(garden: &Garden, start: Coord) -> (Vec<Coord>, Vec<Side>) {
    let width = garden[0].len();
    let height = garden.len();

    let mut perimeter = vec![];
    let nodes = bfs_reach(start, |n| {
        let mut neighbors = n.neighbors();
        perimeter.extend(
            neighbors
                .iter()
                .filter(|c| {
                    !(c.x >= 0
                        && c.x < width as i32
                        && c.y >= 0
                        && c.y < height as i32
                        && garden[c.y as usize][c.x as usize]
                            == garden[start.y as usize][start.x as usize])
                })
                .map(|c| {
                    let facing = *c - *n;
                    Side { pos: *c, facing }
                }),
        );
        neighbors.retain(|c| {
            c.x >= 0
                && c.x < width as i32
                && c.y >= 0
                && c.y < height as i32
                && garden[c.y as usize][c.x as usize] == garden[start.y as usize][start.x as usize]
        });
        neighbors
    });
    (nodes.collect(), perimeter)
}

fn part_one(input: &str) -> Option<usize> {
    let garden = read_input(input);
    let width = garden[0].len();
    let height = garden.len();

    let mut visited = vec![vec![false; width]; height];

    let mut cost = 0;
    for i in 0..height {
        for j in 0..width {
            if !visited[i][j] {
                let (section, perimeter) = find_section(&garden, Coord::new(j as i32, i as i32));
                cost += perimeter.len() * section.len();
                for coord in section {
                    visited[coord.y as usize][coord.x as usize] = true;
                }
            }
        }
    }
    Some(cost)
}

fn count_sides(perimeter: &Vec<Side>) -> usize {
    let points: HashSet<Side> = perimeter.iter().cloned().collect();
    let mut visited: HashSet<Side> = HashSet::new();

    let mut sides = 0;
    for start in perimeter {
        if visited.contains(start) {
            continue;
        }
        sides += 1;
        let nodes = bfs_reach(*start, |c| {
            let neighbors = c.pos.neighbors();
            neighbors
                .iter()
                .map(|c| Side {
                    pos: *c,
                    facing: start.facing,
                })
                .filter(|s| points.contains(s))
                .collect::<Vec<Side>>()
        });
        visited.extend(nodes.into_iter());
        // let nodes = bfs_reach(start.pos, |c| {
        //     let mut neighbors = c.neighbors();
        //     neighbors.retain(|c| (c.manhattan(&start.pos).abs() == 1) && points.contains(c));
        //     neighbors
        // });
        // visited.extend(nodes.into_iter());
    }
    sides
}

fn part_two(input: &str) -> Option<usize> {
    let garden = read_input(input);
    let width = garden[0].len();
    let height = garden.len();

    let mut visited = vec![vec![false; width]; height];

    let mut cost = 0;
    for i in 0..height {
        for j in 0..width {
            if !visited[i][j] {
                let (section, perimeter) = find_section(&garden, Coord::new(j as i32, i as i32));
                let sides = count_sides(&perimeter);
                cost += sides * section.len();
                for coord in section {
                    visited[coord.y as usize][coord.x as usize] = true;
                }
            }
        }
    }
    Some(cost)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 12);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 12);
        assert_eq!(part_one(&input), Some(772));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 12);
        assert_eq!(part_two(&input), Some(436));
    }
}
