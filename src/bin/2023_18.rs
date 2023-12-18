use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;

type Int = i64;
type Coord = Coordinate<Int>;

#[derive(Debug)]
struct Instruction {
    direction: char,
    amount: u32,
}

fn part_one(input: &str) -> Option<Int> {
    let inst = input
        .lines()
        .map(|l| {
            let parts = aoc::utils::read_pattern("{} {} (#{})", l).unwrap();

            Instruction {
                direction: parts[0].chars().next().unwrap(),
                amount: u32::from_str_radix(parts[1], 10).unwrap(),
            }
        })
        .collect_vec();

    Some(area(&inst))
}

fn area(instructions: &Vec<Instruction>) -> Int {
    let mut cursor = Coord::new(0, 0);
    let mut vertices: Vec<Coord> = vec![cursor.clone()];

    let mut perim = 0;
    for i in instructions {
        let direction = match i.direction {
            'U' => Coord::new(0, -1),
            'D' => Coord::new(0, 1),
            'L' => Coord::new(-1, 0),
            'R' => Coord::new(1, 0),
            _ => unreachable!(),
        };
        cursor += direction * Coord::new(i.amount as Int, i.amount as Int);
        vertices.push(cursor.clone());
        perim += i.amount;
    }
    vertices.pop();

    let p1 = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .fold(0, |tot, (a, b)| tot + (a.x * b.y - b.x * a.y));

    p1.abs() / 2 + (perim as Int / 2 + 1)
}

fn part_two(input: &str) -> Option<Int> {
    let inst = input
        .lines()
        .map(|l| {
            let parts = aoc::utils::read_pattern("{} {} (#{})", l).unwrap();

            let am = u32::from_str_radix(&parts[2][0..5], 16).unwrap();
            let dir = match parts[2].chars().nth(5).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };

            Instruction {
                direction: dir,
                amount: am,
            }
        })
        .collect_vec();

    Some(area(&inst))
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 18);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 18);
        assert_eq!(part_one(&input), Some(62));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 18);
        assert_eq!(part_two(&input), Some(952408144115));
    }
}
