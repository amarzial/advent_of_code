fn part_one(input: &str) -> Option<i32> {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(input);

    let mut total = 0;
    for cap in matches {
        match (cap.get(1), cap.get(2)) {
            (Some(a), Some(b)) => {
                let a = a.as_str().parse::<i32>().unwrap();
                let b = b.as_str().parse::<i32>().unwrap();
                total += a * b;
            }
            _ => {}
        }
    }
    Some(total)
}

fn part_two(input: &str) -> Option<i32> {
    let re = regex::Regex::new(r"(?:mul\((\d+),(\d+)\))|(do(?:n't)?\(\))").unwrap();
    let matches = re.captures_iter(input);

    let mut total = 0;
    let mut enabled = true;
    for cap in matches {
        match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(a), Some(b), None) => {
                let a = a.as_str().parse::<i32>().unwrap();
                let b = b.as_str().parse::<i32>().unwrap();
                if enabled {
                    total += a * b;
                }
            }
            (None, None, Some(c)) => match c.as_str() {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 03);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 03);
        assert_eq!(part_one(&input), Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 03);
        assert_eq!(part_two(&input), Some(48));
    }
}
