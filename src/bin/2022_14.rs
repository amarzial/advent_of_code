use std::{collections::HashSet, iter::successors};

use aoc::helpers::coordinate::Coordinate;

type Int = i32;
type Grid = HashSet<Coordinate<i32>>;

fn build_grid(input: &str) -> (Grid, Int) {
    let mut grid = HashSet::new();
    let mut lowest = 0;

    for l in input.lines() {
        let points: Vec<Coordinate<Int>> = l
            .split(" -> ")
            .map(|p| {
                let mut sp = p.split(',');
                Coordinate::new(
                    sp.next().unwrap().parse().unwrap(),
                    sp.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        let mut piter = points.iter();
        let mut cursor = *piter.next().unwrap();
        // grid.insert(cursor);
        for target in piter {
            let moves = *target - cursor;
            for n in successors(Some(cursor), |p| {
                if p == target {
                    None
                } else {
                    Some(*p + Coordinate::new(moves.x.signum(), moves.y.signum()))
                }
            }) {
                lowest = lowest.max(n.y);
                grid.insert(n);
            }
            cursor = *target;
        }
    }
    (grid, lowest)
}

fn place(
    grid: &mut Grid,
    start: Coordinate<Int>,
    bottom: Int,
    part: Int,
) -> Option<Coordinate<Int>> {
    static TRIES: [Coordinate<Int>; 3] = [
        Coordinate { x: 0, y: 1 },
        Coordinate { x: -1, y: 1 },
        Coordinate { x: 1, y: 1 },
    ];

    let mut sand = start;
    if grid.contains(&sand) {
        return None;
    };
    loop {
        let mut movement = false;
        for t in TRIES.iter() {
            let current = sand + *t;
            if !grid.contains(&current) {
                sand = current;
                movement = true;
                break;
            }
        }
        if part == 1 && sand.y > bottom {
            return None;
        } else if part == 2 && sand.y == bottom + 1 {
            break;
        }
        if !movement {
            break;
        }
    }
    grid.insert(sand);
    Some(sand)
}

fn part_one(input: &str) -> Option<Int> {
    let (mut grid, lowest) = build_grid(input);

    let mut steps = 0;
    while let Some(_) = place(&mut grid, Coordinate { x: 500, y: 0 }, lowest, 1) {
        steps += 1;
    }

    Some(steps)
}

fn part_two(input: &str) -> Option<Int> {
    let (mut grid, lowest) = build_grid(input);

    let mut steps = 0;
    while let Some(_) = place(&mut grid, Coordinate { x: 500, y: 0 }, lowest, 2) {
        steps += 1;
    }

    Some(steps)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 14);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
