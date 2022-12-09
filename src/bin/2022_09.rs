use std::collections::HashSet;

type Point = (i32, i32);

fn follow(head: Point, mut tail: Point) -> Point {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() > 1 || dy.abs() > 1 {
        if dx != 0 {
            tail.0 += if dx > 0 { 1 } else { -1 };
        }
        if dy != 0 {
            tail.1 += if dy > 0 { 1 } else { -1 };
        }
    }
    tail
}

fn run(commands: &Vec<(char, i32)>, num_knots: usize) -> usize {
    let mut knots: Vec<Point> = Vec::new();
    knots.resize(num_knots, (0, 0));

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(*knots.last().unwrap());

    for cmd in commands {
        let movement = match cmd.0 {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => (0, 0),
        };

        for _i in 0..cmd.1 {
            let mut p = knots[0];
            p.0 += movement.0;
            p.1 += movement.1;
            knots[0] = p;

            for n in 1..num_knots {
                knots[n] = follow(knots[n - 1], knots[n]);
            }
            visited.insert(*knots.last().unwrap());
        }
    }
    visited.len()
}

fn part_one(commands: &Vec<(char, i32)>) -> Option<usize> {
    Some(run(&commands, 2))
}

fn part_two(commands: &Vec<(char, i32)>) -> Option<usize> {
    Some(run(&commands, 10))
}

fn build_input(input: &str) -> Vec<(char, i32)> {
    aoc::utils::read_list_parse(&input, |l| {
        let dir = l[0..1].chars().next().unwrap();
        let amount = l[2..].parse::<i32>().unwrap();
        (dir, amount)
    })
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 09);
    let commands = build_input(&input);

    aoc::solve!(1, part_one, &commands);
    aoc::solve!(2, part_two, &commands);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 09);
        let commands = build_input(&input);
        assert_eq!(part_one(&commands), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 09);
        let commands = build_input(&input);
        assert_eq!(part_two(&commands), Some(1));
    }
}
