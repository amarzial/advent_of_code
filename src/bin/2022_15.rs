use std::collections::HashSet;

use aoc::helpers::coordinate::Coordinate;

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

fn part_one(input: &str) -> Option<usize> {
    let points = aoc::utils::read_list_parse(input, parse_line);

    let row_number = if cfg!(test) { 10 } else { 2e6 as i32 };

    let mut ranges = Vec::new();
    let mut beacons = HashSet::new();

    for pair in points.iter() {
        if pair.0.y == row_number {
            ranges.push(pair.0.x..pair.0.x + 1);
        }
        if pair.1.y == row_number {
            beacons.insert(pair.1);
        }
    }

    for pair in points.iter() {
        let dist = pair.0.manhattan(&pair.1);
        let closest_on_row = Coord::new(pair.0.x, row_number);
        if dist >= pair.0.manhattan(&closest_on_row) {
            let dist_excess = dist - pair.0.manhattan(&closest_on_row);
            ranges.push(closest_on_row.x - dist_excess..closest_on_row.x + dist_excess + 1);
        }
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = Vec::new();
    result.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let current = ranges[i].clone();
        let last = result.len() - 1;
        if result[last].end >= current.start {
            result[last] = result[last].start..(result[last].end.max(current.end));
        } else {
            result.push(current);
        }
    }

    let out = result.iter().fold(0, |tot, current| tot + current.len());
    Some(out - beacons.len())
}

fn part_two(input: &str) -> Option<usize> {
    let points = aoc::utils::read_list_parse(input, parse_line);

    let upper_bound = if cfg!(test) { 20 } else { 4e6 as i32 };

    for row_number in 0_i32..upper_bound {
        // println!("Row: {}", row_number);
        let mut ranges = Vec::new();

        for pair in points.iter() {
            let dist = pair.0.manhattan(&pair.1);
            let closest_on_row = Coord::new(pair.0.x, row_number);
            if dist >= pair.0.manhattan(&closest_on_row) {
                let dist_excess = dist - pair.0.manhattan(&closest_on_row);
                let low = 0.max(closest_on_row.x - dist_excess);
                let high = upper_bound.min(closest_on_row.x + dist_excess + 1);
                ranges.push(low..high);
            }
        }

        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let mut result = Vec::new();
        result.push(ranges[0].clone());

        for i in 1..ranges.len() {
            let current = ranges[i].clone();
            let last = result.len() - 1;
            if result[last].end >= current.start {
                result[last] = result[last].start..(result[last].end.max(current.end));
            } else {
                result.push(current);
            }
        }
        // println!("ranges: {:?}", result);

        let out = result.iter().fold(0, |tot, current| tot + current.len());
        if out < upper_bound as usize {
            return Some(result[0].end as usize * 4e6 as usize + row_number as usize);
        }
    }
    Some(0)
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
        assert_eq!(part_two(&input), Some(56000011));
    }
}
