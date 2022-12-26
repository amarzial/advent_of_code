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
    let mut count = 0;
    let mut which = (0..4).filter(|c| {
        let cur = (current + *c) as usize % 4;
        for y in MOVES[cur].0.clone() {
            for x in MOVES[cur].1.clone() {
                if field.contains(&(elf + C::new(x, y))) {
                    return false;
                }
            }
        }
        count += 1;
        true
    });

    let found = which.next();
    while let Some(_) = which.next() {}
    if count == 4 {
        return None;
    }

    if let Some(d) = found {
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

fn grid_size(field: &Grid) -> (C, C) {
    let mut minx = Int::MAX;
    let mut miny = Int::MAX;
    let mut maxx = Int::MIN;
    let mut maxy = Int::MIN;

    for c in field {
        minx = minx.min(c.x);
        miny = miny.min(c.y);
        maxx = maxx.max(c.x);
        maxy = maxy.max(c.y);
    }

    (C::new(minx, miny), C::new(maxx, maxy))
}

fn _print(field: &Grid) {
    let limits = grid_size(field);
    for y in limits.0.y..=limits.1.y {
        for x in limits.0.x..=limits.1.x {
            print!(
                "{}",
                if field.contains(&C::new(x, y)) {
                    '#'
                } else {
                    '.'
                }
            )
        }
        println!("");
    }
    println!("");
}

fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse_grid(input);

    for current in 0..10 {
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
        for p in possible.iter() {
            grid.remove(&p.1[0]);
            grid.insert(*p.0);
        }
    }
    let limits = grid_size(&grid);
    let area = (limits.0.x.abs_diff(limits.1.x) + 1) * (limits.0.y.abs_diff(limits.1.y) + 1);
    Some(area as usize - grid.len())
}

fn part_two(input: &str) -> Option<Int> {
    let mut grid = parse_grid(input);

    let mut current = 0;
    let mut moved = true;
    while moved {
        moved = false;
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
        if possible.len() > 0 {
            moved = true;
        }
        for p in possible.iter() {
            grid.remove(&p.1[0]);
            grid.insert(*p.0);
        }
        current += 1;
    }
    Some(current)
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
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
