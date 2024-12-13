use aoc::{helpers::coordinate::Coordinate, utils::read_pattern};

type Coord = Coordinate<i64>;
#[derive(Debug)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Machine {
    fn find_cost(&self) -> Option<usize> {
        let a = (self.prize.x * self.b.y - self.prize.y * self.b.x) as f64
            / (self.a.x * self.b.y - self.a.y * self.b.x) as f64;
        if a != a.floor() {
            return None;
        }
        let b = (self.prize.x as f64 - (a * self.a.x as f64)) / self.b.x as f64;

        if b != b.floor() {
            return None;
        }
        Some(a as usize * 3 + (b as usize))
    }
}

fn read_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();
            let a = read_pattern("Button A: X+{}, Y+{}", lines.next().unwrap()).unwrap();
            let b = read_pattern("Button B: X+{}, Y+{}", lines.next().unwrap()).unwrap();
            let prize = read_pattern("Prize: X={}, Y={}", lines.next().unwrap()).unwrap();

            Machine {
                a: Coord::new(a[0].parse().unwrap(), a[1].parse().unwrap()),
                b: Coord::new(b[0].parse().unwrap(), b[1].parse().unwrap()),
                prize: Coord::new(prize[0].parse().unwrap(), prize[1].parse().unwrap()),
            }
        })
        .collect()
}

fn part_one(input: &str) -> Option<usize> {
    let machines = read_input(input);

    let mut total = 0;
    for machine in machines {
        if let Some(cost) = machine.find_cost() {
            total += cost;
        }
    }
    Some(total)
}

fn part_two(input: &str) -> Option<usize> {
    let machines = read_input(input);

    let mut total = 0;
    for mut machine in machines {
        machine.prize.x += 1e13 as i64;
        machine.prize.y += 1e13 as i64;
        if let Some(cost) = machine.find_cost() {
            total += cost;
        }
    }
    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 13);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 13);
        assert_eq!(part_one(&input), Some(480));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 13);
        assert_eq!(part_two(&input), None);
    }
}
