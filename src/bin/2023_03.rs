use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;
type Coord = Coordinate<usize>;

fn rewind(grid: &Vec<Vec<char>>, coord: Coordinate<usize>) -> Coordinate<usize> {
    let mut cur = coord.clone();
    while cur.x > 0 && grid[cur.y][cur.x - 1].is_digit(10) {
        cur.x -= 1;
    }
    cur
}

fn find_parts(grid: &Vec<Vec<char>>) -> HashSet<Coordinate<usize>> {
    let mut parts = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let chr = grid[row][col];
            if !chr.is_digit(10) && chr != '.' {
                for coord in Coordinate::new(col, row).neighbors_extended() {
                    if coord.x < grid[0].len()
                        && coord.y < grid.len()
                        && grid[coord.y][coord.x].is_digit(10)
                    {
                        parts.insert(coord);
                    }
                }
            }
        }
    }
    parts
}

fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = aoc::utils::read_list_parse(input, |l| l.chars().collect());

    let mut parts = find_parts(&grid);

    let mut sum = 0;
    for coord in parts.clone().iter().sorted_by(|a, b| {
        if a.y < b.y {
            return Ordering::Less;
        } else if a.y > b.y {
            return Ordering::Greater;
        } else {
            return Ord::cmp(&a.x, &b.x);
        }
    }) {
        if !parts.contains(&coord) {
            continue;
        }
        let mut cursor = rewind(&grid, coord.clone());
        let mut num = 0;
        while grid[cursor.y][cursor.x].is_digit(10) {
            num *= 10;
            num += grid[cursor.y][cursor.x].to_digit(10).unwrap();

            if cursor.x < grid[0].len() - 1 {
                parts.remove(&cursor);
                cursor.x += 1;
            } else {
                break;
            }
        }
        sum += num;
    }

    Some(sum)
}

fn find_gears(grid: &Vec<Vec<char>>) -> HashMap<Coord, HashSet<Coord>> {
    let mut gears: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let chr = grid[row][col];
            if chr == '*' {
                let gear = Coordinate::new(col, row);
                for coord in gear.neighbors_extended() {
                    if coord.x < grid[0].len()
                        && coord.y < grid.len()
                        && grid[coord.y][coord.x].is_digit(10)
                    {
                        if !gears.contains_key(&gear) {
                            gears.insert(gear, HashSet::new());
                        }
                        gears.get_mut(&gear).unwrap().insert(coord.clone());
                    }
                }
            }
        }
    }
    gears
}

fn gear_ratio(grid: &Vec<Vec<char>>, mut gear: HashSet<Coord>) -> usize {
    let mut ratio = 1;
    let mut cnt = 0;

    for coord in gear.clone().iter().sorted_by(|a, b| {
        if a.y < b.y {
            return Ordering::Less;
        } else if a.y > b.y {
            return Ordering::Greater;
        } else {
            return Ord::cmp(&a.x, &b.x);
        }
    }) {
        if !gear.contains(&coord) {
            continue;
        }
        cnt += 1;
        let mut cursor = rewind(&grid, coord.clone());
        let mut num = 0;
        while grid[cursor.y][cursor.x].is_digit(10) {
            num *= 10;
            num += grid[cursor.y][cursor.x].to_digit(10).unwrap();

            if cursor.x < grid[0].len() - 1 {
                gear.remove(&cursor);
                cursor.x += 1;
            } else {
                break;
            }
        }
        ratio *= num as usize;
    }

    if cnt <= 1 {
        0
    } else {
        ratio
    }
}

fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = aoc::utils::read_list_parse(input, |l| l.chars().collect());

    let gears = find_gears(&grid);

    let mut sum = 0;
    for (_, gear) in gears {
        let ratio = gear_ratio(&grid, gear.clone());
        sum += ratio;
    }
    Some(sum)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 03);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 03);
        assert_eq!(part_one(&input), Some(4361));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 03);
        assert_eq!(part_two(&input), Some(467835));
    }
}
