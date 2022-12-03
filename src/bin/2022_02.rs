fn play_score(elf: char, player: char) -> i32 {
    match elf {
        'A' => match player {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            _ => 0,
        },
        'B' => match player {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0,
        },
        'C' => match player {
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            _ => 0,
        },
        _ => 0,
    }
}

fn strategy_score(elf: char, player: char) -> i32 {
    match elf {
        'A' => match player {
            'X' => 0 + 3,
            'Y' => 3 + 1,
            'Z' => 6 + 2,
            _ => 0,
        },
        'B' => match player {
            'X' => 0 + 1,
            'Y' => 3 + 2,
            'Z' => 6 + 3,
            _ => 0,
        },
        'C' => match player {
            'X' => 0 + 2,
            'Y' => 3 + 3,
            'Z' => 6 + 1,
            _ => 0,
        },
        _ => 0,
    }
}

fn part_one(input: &str) -> Option<String> {
    let actions = aoc::utils::read_list_parse(input, |s| {
        let mut c = s.chars();
        (c.next().unwrap(), c.nth(1).unwrap())
    });

    let mut score = 0;
    for a in actions {
        score += match a.1 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };
        score += play_score(a.0, a.1);
    }
    Some(score.to_string())
}

fn part_two(input: &str) -> Option<String> {
    let actions = aoc::utils::read_list_parse(input, |s| {
        let mut c = s.chars();
        (c.next().unwrap(), c.nth(1).unwrap())
    });

    let mut score = 0;
    for a in actions {
        score += strategy_score(a.0, a.1);
    }
    Some(score.to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 02);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 02);
        assert_eq!(part_one(&input), Some(15.to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 02);
        assert_eq!(part_two(&input), Some(12.to_string()));
    }
}
