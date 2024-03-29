use std::mem::swap;

type Pair = (i32, i32);

fn parse_line(line: &str) -> (Pair, Pair) {
    let numbers: Vec<i32> = aoc::utils::read_pattern("{}-{},{}-{}", line)
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

fn is_contained(p1: Pair, p2: Pair) -> bool {
    if (p1.0 >= p2.0 && p1.1 <= p2.1) || (p2.0 >= p1.0 && p2.1 <= p1.1) {
        return true;
    };
    false
}

fn is_overlap(mut p1: Pair, mut p2: Pair) -> bool {
    if p1.0 > p2.0 {
        swap(&mut p1, &mut p2);
    }
    if p1.1 >= p2.0 {
        return true;
    }
    false
}

fn part_one(input: &str) -> Option<i32> {
    let pairs = aoc::utils::read_list_parse(input, parse_line);

    let res = pairs.iter().fold(0, |tot, p| {
        tot + match is_contained(p.0, p.1) {
            true => 1,
            false => 0,
        }
    });
    Some(res)
}

fn part_two(input: &str) -> Option<i32> {
    let pairs = aoc::utils::read_list_parse(input, parse_line);

    let res = pairs.iter().fold(0, |tot, p| {
        tot + match is_overlap(p.0, p.1) {
            true => 1,
            false => 0,
        }
    });
    Some(res)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 04);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 04);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 04);
        assert_eq!(part_two(&input), Some(4));
    }
}
