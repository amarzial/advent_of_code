use std::collections::HashSet;

use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;
use pathfinding::directed::bfs;

type Int = i32;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Beam {
    direction: Coordinate<Int>,
    position: Coordinate<Int>,
}

impl Beam {
    fn next_position(&self, grid: &Vec<Vec<char>>) -> Vec<Beam> {
        let current = grid[self.position.y as usize][self.position.x as usize];
        let check_bounds = |new_pos: Coordinate<Int>| -> bool {
            new_pos.x >= 0
                && new_pos.x < grid[0].len() as Int
                && new_pos.y >= 0
                && new_pos.y < grid.len() as Int
        };

        let follow = |new_dir| {
            let new_pos = self.position + new_dir;
            if check_bounds(new_pos) {
                vec![Beam {
                    direction: new_dir,
                    position: new_pos,
                }]
            } else {
                vec![]
            }
        };

        let split = |current_dir: Coordinate<Int>| -> (Coordinate<Int>, Coordinate<Int>) {
            let dir1 = Coordinate::new(current_dir.y, current_dir.x);
            let dir2 = Coordinate::new(current_dir.y * -1, current_dir.x * -1);
            (dir1, dir2)
        };
        match current {
            '.' | '/' | '\\' => {
                let new_dir = if current == '/' {
                    Coordinate::new(self.direction.y * -1, self.direction.x * -1)
                } else if current == '\\' {
                    Coordinate::new(self.direction.y, self.direction.x)
                } else {
                    self.direction
                };
                follow(new_dir)
            }
            '|' => {
                if self.direction.y != 0 {
                    follow(self.direction)
                } else {
                    let (d1, d2) = split(self.direction);
                    let mut dest = follow(d1);
                    dest.extend(follow(d2).into_iter());
                    dest
                }
            }
            '-' => {
                if self.direction.x != 0 {
                    follow(self.direction)
                } else {
                    let (d1, d2) = split(self.direction);
                    let mut dest = follow(d1);
                    dest.extend(follow(d2).into_iter());
                    dest
                }
            }
            _ => vec![],
        }
    }
}

fn run(start: Beam, grid: &Vec<Vec<char>>) -> usize {
    let cells = bfs::bfs_reach(start, |b| {
        let n = b.next_position(&grid);
        n
    });

    let set: HashSet<Coordinate<Int>> = HashSet::from_iter(cells.map(|b| b.position));
    set.len()
}

fn part_one(input: &str) -> Option<usize> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let start = Beam {
        direction: Coordinate::new(1, 0),
        position: Coordinate::new(0, 0),
    };

    Some(run(start, &grid))
}

fn part_two(input: &str) -> Option<usize> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let start_beams = (0..grid.len())
        .map(|i| Beam {
            direction: Coordinate::new(1, 0),
            position: Coordinate::new(0, i as Int),
        })
        .chain((0..grid.len()).map(|i| Beam {
            direction: Coordinate::new(-1, 0),
            position: Coordinate::new((grid[0].len() - 1) as Int, i as Int),
        }))
        .chain((0..grid[0].len()).map(|i| Beam {
            direction: Coordinate::new(0, 1),
            position: Coordinate::new(i as Int, 0),
        }))
        .chain((0..grid[0].len()).map(|i| Beam {
            direction: Coordinate::new(0, -1),
            position: Coordinate::new(i as Int, (grid.len() - 1) as Int),
        }));

    Some(start_beams.map(|b| run(b, &grid)).max().unwrap())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 16);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 16);
        assert_eq!(part_one(&input), Some(46));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 16);
        assert_eq!(part_two(&input), Some(51));
    }
}
