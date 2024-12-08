use std::collections::HashSet;

use aoc::helpers::coordinate::Coordinate;

type Coord = Coordinate<i32>;

#[derive(Debug)]
struct Antenna {
    coord: Coord,
    strength: char,
}

fn load_input(input: &str) -> Vec<Antenna> {
    let antennas: Vec<Antenna> = input
        .lines()
        .enumerate()
        .map(|line| {
            let y = line.0.clone();
            line.1.chars().enumerate().map(move |c| {
                let x = c.0.clone();
                Antenna {
                    coord: Coord::new(x as i32, y as i32),
                    strength: c.1,
                }
            })
        })
        .flatten()
        .filter(|a| a.strength != '.')
        .collect();
    antennas
}

fn part_one(input: &str) -> Option<usize> {
    let antennas = load_input(input);
    let mut antinodes = HashSet::new();
    let rows = input.lines().next().unwrap().len();
    let cols = input.lines().count();

    for antenna in antennas.iter() {
        let paired = antennas
            .iter()
            .filter(|a| a.coord != antenna.coord && a.strength == antenna.strength);

        for pair in paired {
            let diff = antenna.coord - pair.coord;

            let antinode1 = antenna.coord + diff;
            let antinode2 = pair.coord - diff;
            antinodes.insert(antinode1);
            antinodes.insert(antinode2);
        }
    }
    let valid = antinodes
        .iter()
        .filter(|a| a.x >= 0 && a.y >= 0 && a.x < rows as i32 && a.y < cols as i32);
    Some(valid.count())
}

fn part_two(input: &str) -> Option<usize> {
    let antennas = load_input(input);
    let mut antinodes = HashSet::new();
    let rows = input.lines().next().unwrap().len();
    let cols = input.lines().count();

    for antenna in antennas.iter() {
        let paired = antennas
            .iter()
            .filter(|a| a.coord != antenna.coord && a.strength == antenna.strength);

        for pair in paired {
            let diff = antenna.coord - pair.coord;

            let mut antinode1 = antenna.coord;
            let mut antinode2 = pair.coord;

            loop {
                if antinode1.x < 0
                    || antinode1.y < 0
                    || antinode1.x >= rows as i32
                    || antinode1.y >= cols as i32
                {
                    break;
                }
                antinodes.insert(antinode1);
                antinode1 = antinode1 + diff;
            }

            loop {
                if antinode2.x < 0
                    || antinode2.y < 0
                    || antinode2.x >= rows as i32
                    || antinode2.y >= cols as i32
                {
                    break;
                }
                antinodes.insert(antinode2);
                antinode2 = antinode2 - diff;
            }
        }
    }
    Some(antinodes.len())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 08);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 08);
        assert_eq!(part_one(&input), Some(14));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 08);
        assert_eq!(part_two(&input), Some(34));
    }
}
