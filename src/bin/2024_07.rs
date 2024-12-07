use aoc::utils::{read_list_parse, read_pattern};
use pathfinding::directed::bfs;

type Int = i64;
type Operation = (Int, Vec<Int>);

fn parse_input(input: &str) -> Vec<Operation> {
    let list: Vec<Operation> = read_list_parse(input, |l| {
        let values = read_pattern("{}: {}", l).unwrap();
        (
            values[0].parse().unwrap(),
            values[1]
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        )
    });
    list
}

fn is_valid(op: &Operation, part2: bool) -> bool {
    let start = op.1[0];
    let res = bfs::bfs(
        &(start, 0),
        |n| {
            if n.1 == op.1.len() - 1 || n.0 > op.0 {
                return vec![];
            }
            if part2 {
                let mut str = n.0.to_string();
                str.push_str(op.1[n.1 + 1].to_string().as_str());
                return vec![
                    (n.0 + op.1[n.1 + 1], n.1 + 1),
                    (n.0 * op.1[n.1 + 1], n.1 + 1),
                    (str.parse().unwrap(), n.1 + 1),
                ];
            } else {
                return vec![
                    (n.0 + op.1[n.1 + 1], n.1 + 1),
                    (n.0 * op.1[n.1 + 1], n.1 + 1),
                ];
            }
        },
        |n| (n.1 == op.1.len() - 1) && (n.0 == op.0),
    );
    return res.is_some();
}

fn part_one(input: &str) -> Option<Int> {
    let list = parse_input(input);
    let mut tot = 0;
    for op in list {
        if is_valid(&op, false) {
            tot += op.0;
        }
    }
    Some(tot)
}

fn part_two(input: &str) -> Option<Int> {
    let list = parse_input(input);
    let mut tot = 0;
    for op in list {
        if is_valid(&op, true) {
            tot += op.0;
        }
    }
    Some(tot)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 07);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 07);
        assert_eq!(part_one(&input), Some(3749));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 07);
        assert_eq!(part_two(&input), Some(11387));
    }
}
