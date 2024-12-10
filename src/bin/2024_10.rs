use std::vec;

use aoc::helpers::coordinate::Coordinate;
use pathfinding::prelude::{bfs, bfs_reach};

fn load_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

type Coord = Coordinate<i32>;

fn start_points(grid: &Vec<Vec<i32>>) -> Vec<Coord> {
    let width = grid[0].len();
    let height = grid.len();
    let mut starts = vec![];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                starts.push(Coord::new(x as i32, y as i32));
            }
        }
    }
    starts
}

fn part_one(input: &str) -> Option<usize> {
    let grid = load_input(input);
    let width = grid[0].len();
    let height = grid.len();
    let starts = start_points(&grid);

    let score = starts
        .iter()
        .map(|start| {
            let nodes = bfs_reach(*start, |f| {
                let from = grid[f.y as usize][f.x as usize];
                f.neighbors()
                    .into_iter()
                    .filter(|n| {
                        if n.x >= 0 && n.x < width as i32 && n.y >= 0 && n.y < height as i32 {
                            let to = grid[n.y as usize][n.x as usize];
                            return to - from == 1;
                        }
                        false
                    })
                    .collect::<Vec<Coord>>()
            });
            nodes.fold(0, |acc, node| {
                acc + if grid[node.y as usize][node.x as usize] == 9 {
                    1
                } else {
                    0
                }
            })
        })
        .sum();
    Some(score)
}

fn part_two(input: &str) -> Option<usize> {
    let grid = load_input(input);
    let width = grid[0].len();
    let height = grid.len();
    let starts = start_points(&grid);

    let score = starts
        .iter()
        .map(|start| {
            let mut path_count = 0;
            bfs(
                start,
                |f| {
                    let from = grid[f.y as usize][f.x as usize];
                    f.neighbors()
                        .into_iter()
                        .filter(|n| {
                            if n.x >= 0 && n.x < width as i32 && n.y >= 0 && n.y < height as i32 {
                                let to = grid[n.y as usize][n.x as usize];
                                return to - from == 1;
                            }
                            false
                        })
                        .collect::<Vec<Coord>>()
                },
                |f| {
                    let to = grid[f.y as usize][f.x as usize];
                    if to == 9 {
                        path_count += 1;
                    }
                    false
                },
            );
            path_count
        })
        .sum();
    Some(score)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 10);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 10);
        assert_eq!(part_one(&input), Some(36));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 10);
        assert_eq!(part_two(&input), Some(81));
    }
}
