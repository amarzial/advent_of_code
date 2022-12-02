fn part_one(input: &str) -> Option<String> {
    let lines = aoc::utils::read_list_parse(&input, |s| s.parse::<i32>());

    let mut current = 0;
    let mut max = 0;
    for line in lines {
        if line.is_ok() {
            current += line.unwrap();
        } else {
            max = max.max(current);
            current = 0;
        }
    }
    max = max.max(current);

    return Some(max.to_string());
}

fn part_two(input: &str) -> Option<String> {
    let lines = aoc::utils::read_list_parse(&input, |s| s.parse::<i32>());

    let mut elves = Vec::new();
    let mut current = 0;
    for line in lines {
        if line.is_ok() {
            current += line.unwrap();
        } else {
            elves.push(current);
            current = 0;
        }
    }
    elves.push(current);

    elves.sort();
    let sum: i32 = elves[elves.len() - 3..].into_iter().sum();

    return Some(sum.to_string());
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 01);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 01);
        assert_eq!(part_one(&input), Some(24000.to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 01);
        assert_eq!(part_two(&input), Some(45000.to_string()));
    }
}
