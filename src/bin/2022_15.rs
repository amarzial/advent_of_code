use std::collections::{HashMap, HashSet};

use aoc::helpers::Coordinate;

type Coord = Coordinate<i32>;

fn parse_line(l: &str) -> (Coord, Coord) {
    let pts = aoc::utils::read_pattern("Sensor at x={}, y={}: closest beacon is at x={}, y={}", l)
        .unwrap();
    let mut it = pts.iter();
    (
        Coord::new(
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        ),
        Coord::new(
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        ),
    )
}

fn part_one(input: &str) -> Option<i32> {
    let points = aoc::utils::read_list_parse(input, parse_line);

    let row_number = 2000000;
    let mut row = HashMap::new();

    for pair in points.iter() {
        if pair.0.y == row_number {
            row.insert(pair.0, 1);
        }
        if pair.1.y == row_number {
            row.insert(pair.1, 0);
        }
    }

    for pair in points.iter() {
        let dist = pair.0.manhattan(&pair.1);
        let closest_on_row = Coord::new(pair.0.x, row_number);
        let mut cursor = closest_on_row;
        if dist >= pair.0.manhattan(&closest_on_row) {
            let dist_excess = dist - pair.0.manhattan(&closest_on_row);
            for i in (closest_on_row.x - dist_excess)..closest_on_row.x {
                cursor.x = i;
                if !row.contains_key(&cursor) {
                    row.insert(cursor, 1);
                }
            }
            cursor = closest_on_row;
            for i in closest_on_row.x..(closest_on_row.x + dist_excess + 1) {
                cursor.x = i;
                if !row.contains_key(&cursor) {
                    row.insert(cursor, 1);
                }
            }
        }
    }

    let out = row.values().sum();

    Some(out)
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 15);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 15);
        assert_eq!(part_two(&input), None);
    }
}
