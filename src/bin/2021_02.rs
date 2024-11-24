use aoc::utils::cache;

use aoc::utils;
enum Direction {
    Down(i32),
    Up(i32),
    Forward(i32),
}

fn parse_direction(line: &str) -> Direction {
    let words: Vec<&str> = line.split(' ').collect();
    match words[0] {
        "down" => {
            return Direction::Down(str::parse(words[1]).unwrap());
        }
        "up" => {
            return Direction::Up(str::parse(words[1]).unwrap());
        }
        "forward" => {
            return Direction::Forward(str::parse(words[1]).unwrap());
        }
        _ => panic!(),
    }
}

fn part_one(input: &str) -> Option<String> {
    let directions: Vec<Direction> = utils::read_list_parse(input, parse_direction);

    let mut x = 0;
    let mut y = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut aim2 = 0;
    for dir in directions.iter() {
        match *dir {
            Direction::Down(n) => {
                y += n;
                aim2 += n;
            }
            Direction::Up(n) => {
                y -= n;
                aim2 -= n;
            }
            Direction::Forward(n) => {
                x += n;
                x2 += n;
                y2 += n * aim2;
            }
        }
    }

    let p1 = (x * y).to_string();
    cache(Some((x2 * y2).to_string()));

    Some(p1)
}

fn part_two(_input: &str) -> Option<String> {
    cache(None)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2021, 02);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2021, 02);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2021, 02);
        assert_eq!(part_two(&input), None);
    }
}
