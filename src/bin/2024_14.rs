use aoc::{helpers::coordinate::Coordinate, utils::read_pattern};
use itertools::Itertools;

type Coord = Coordinate<i32>;
struct Robot {
    pos: Coord,
    speed: Coord,
}

impl Robot {
    fn step(&mut self, bounds: Coord) {
        self.pos += self.speed;
        if self.pos.x < 0 {
            self.pos.x += bounds.x;
        } else if self.pos.x >= bounds.x {
            self.pos.x -= bounds.x;
        }

        if self.pos.y < 0 {
            self.pos.y += bounds.y;
        } else if self.pos.y >= bounds.y {
            self.pos.y -= bounds.y;
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let values = read_pattern("p={},{} v={},{}", line)
                .unwrap()
                .into_iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec();
            Robot {
                pos: Coord::new(values[0], values[1]),
                speed: Coord::new(values[2], values[3]),
            }
        })
        .collect()
}

fn _print_robots(robots: &[Robot], bound: Coord) {
    let mut grid = vec![vec!['.'; bound.x as usize]; bound.y as usize];
    for robot in robots {
        grid[robot.pos.y as usize][robot.pos.x as usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn count_score(robots: &[Robot], bound: Coord) -> i32 {
    let mut sections = vec![0, 0, 0, 0];
    for r in robots {
        if r.pos.x < bound.x / 2 && r.pos.y < bound.y / 2 {
            sections[0] += 1;
        } else if r.pos.x > bound.x / 2 && r.pos.y < bound.y / 2 {
            sections[1] += 1;
        } else if r.pos.x < bound.x / 2 && r.pos.y > bound.y / 2 {
            sections[2] += 1;
        } else if r.pos.x > bound.x / 2 && r.pos.y > bound.y / 2 {
            sections[3] += 1;
        }
    }
    sections.iter().fold(1, |acc, x| acc * x)
}

fn is_suspect(robots: &[Robot], bound: Coord) -> bool {
    let mut grid = vec![vec!['.'; bound.x as usize]; bound.y as usize];
    for robot in robots {
        grid[robot.pos.y as usize][robot.pos.x as usize] = '#';
    }

    for row in grid {
        let r = String::from_iter(row.iter());
        if r.contains("###############################") {
            return true;
        }
    }
    false
}

fn part_one(input: &str) -> Option<i32> {
    let mut robots = parse_input(input);
    // let bound = Coord::new(11, 7);//test
    let bound = Coord::new(101, 103);

    for _i in 0..100 {
        for robot in &mut robots {
            robot.step(bound);
        }
    }
    // _print_robots(&robots, bound);
    Some(count_score(&robots, bound))
}

fn part_two(input: &str) -> Option<i32> {
    let mut robots = parse_input(input);
    // let bound = Coord::new(11, 7);//test
    let bound = Coord::new(101, 103);

    for i in 0..10000 {
        for robot in &mut robots {
            robot.step(bound);
        }
        if is_suspect(&robots, bound) {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 14);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 14);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 14);
        assert_eq!(part_two(&input), None);
    }
}
