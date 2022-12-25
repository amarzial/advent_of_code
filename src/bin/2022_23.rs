use std::collections::{HashMap, HashSet};

use aoc::helpers::coordinate::Coordinate;
use std::ops::RangeInclusive;

type Int = i32;
type C = Coordinate<Int>;
type Grid = HashSet<C>;

fn parse_grid(input: &str) -> Grid {
    let mut grid = Grid::new();
    for l in input.lines().enumerate() {
        for c in l.1.chars().enumerate() {
            match c.1 {
                '#' => {
                    grid.insert(C::new(c.0 as Int, l.0 as Int));
                }
                _ => {}
            }
        }
    }

    grid
}
static MOVES: [(RangeInclusive<Int>, RangeInclusive<Int>); 4] = [
    (-1..=-1, -1..=1),
    (1..=1, -1..=1),
    (-1..=1, -1..=-1),
    (-1..=1, 1..=1),
];

fn next(elf: C, field: &Grid, current: Int) -> Option<C> {
    let which = (0..4).find(|c| {
        let cur = (current + *c) as usize % 4;
        for y in MOVES[cur].0.clone() {
            for x in MOVES[cur].1.clone() {
                if field.contains(&C::new(x, y)) {
                    return false;
                }
            }
        }
        true
    });

    if let Some(d) = which {
        return match (d + current) % 4 {
            0 => Some(elf + C::new(0, -1)),
            1 => Some(elf + C::new(0, 1)),
            2 => Some(elf + C::new(-1, 0)),
            3 => Some(elf + C::new(1, 0)),
            _ => panic!(""),
        };
    }

    None
}

fn part_one(input: &str) -> Option<String> {
    let mut grid = parse_grid(input);

    let mut current = 0;
    {
        let mut possible: HashMap<C, Vec<C>> = HashMap::new();
        for elf in grid.iter() {
            match next(*elf, &grid, current) {
                Some(n) => {
                    if !possible.contains_key(&n) {
                        possible.insert(n, vec![]);
                    }
                    possible.get_mut(&n).unwrap().push(*elf);
                }
                None => {}
            }
        }
        possible.retain(|_, v| v.len() < 2);
    }

    None
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 23);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 23);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 23);
        assert_eq!(part_two(&input), None);
    }
}
