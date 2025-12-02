use std::{cmp::Ordering, collections::HashSet, vec};

use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;
use pathfinding::prelude::astar;

type Coord = Coordinate<i32>;

fn load_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|f| {
            let mut sp = f.split(",");
            Coord::new(
                sp.next().unwrap().parse().unwrap(),
                sp.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
fn _print_map(map: &HashSet<Coord>, width: usize, height: usize, path: &[Coord]) {
    let p: HashSet<Coord> = HashSet::from_iter(path.into_iter().cloned());
    for y in 0..height {
        for x in 0..width {
            let coord = Coord::new(x as i32, y as i32);
            print!(
                "{}",
                if p.contains(&coord) {
                    'O'
                } else if map.contains(&coord) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }
}

fn part_one(input: &str) -> Option<i32> {
    let width = 71;
    let height = 71;
    let pts = load_input(input);
    let target = Coord::new(width - 1, height - 1);
    let start = Coord::new(0, 0);
    let points: HashSet<Coord> = HashSet::from_iter(pts.iter().take(1024).cloned());
    let path = astar(
        &start,
        |f| {
            f.neighbors()
                .into_iter()
                .filter(|n| {
                    !points.contains(n) && n.x >= 0 && n.y >= 0 && n.x < width && n.y < height
                })
                .map(|n| (n, 1))
                .collect_vec()
        },
        |f| target.manhattan(f),
        |n| *n == target,
    );
    let sol = path.unwrap();

    // _print_map(&points, width as usize, height as usize, &sol.0);

    Some(sol.1)
}

fn blocked(width: i32, height: i32, pts: &Vec<Coord>, steps: usize) -> Option<Coord> {
    let target = Coord::new(width - 1, height - 1);
    let start = Coord::new(0, 0);

    let points: HashSet<Coord> = HashSet::from_iter(pts.iter().take(steps).cloned());
    let path = astar(
        &start,
        |f| {
            f.neighbors()
                .into_iter()
                .filter(|n| {
                    !points.contains(n) && n.x >= 0 && n.y >= 0 && n.x < width && n.y < height
                })
                .map(|n| (n, 1))
                .collect_vec()
        },
        |f| target.manhattan(f),
        |n| *n == target,
    );
    match path {
        Some(_) => None,
        None => Some(pts[steps - 1]),
    }
}

fn binary_search(start: usize, end: usize, found: &dyn Fn(usize) -> Ordering) -> Option<usize> {
    let mut left = start;
    let mut right = end;
    loop {
        let pivot = (right + left) / 2;
        match found(pivot) {
            Ordering::Less => {
                left = pivot;
            }
            Ordering::Greater => {
                right = pivot;
            }
            Ordering::Equal => {
                return Some(pivot);
            }
        }
        if left == right {
            return None;
        }
    }
}

fn part_two(input: &str) -> Option<String> {
    let width = 71;
    let height = 71;
    let pts = load_input(input);

    let index = binary_search(1, pts.len(), &|l| {
        let blk = blocked(width, height, &pts, l);
        if blk.is_some() {
            let blk2 = blocked(width, height, &pts, l - 1);
            if blk2.is_none() {
                return Ordering::Equal;
            }
            return Ordering::Greater;
        }

        Ordering::Less
    });

    let coord = pts[index.unwrap() - 1];

    Some(format!("{},{}", coord.x, coord.y))
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 18);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 18);
        assert_eq!(part_one(&input), Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 18);
        assert_eq!(part_two(&input), Some(String::from("6,1")));
    }
}
