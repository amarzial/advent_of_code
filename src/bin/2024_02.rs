type Level = Vec<i32>;

fn get_levels(input: &str) -> Vec<Level> {
    aoc::utils::read_list_parse(input, |f| {
        f.split_whitespace().map(|x| x.parse().unwrap()).collect()
    })
}

fn is_safe(level: &Level) -> bool {
    let mut prev = level[0];
    let increasing = level[0] < level[1];
    for n in level[1..].iter() {
        if increasing && prev > *n {
            return false;
        }
        if !increasing && prev < *n {
            return false;
        }

        if (prev == *n) || (prev.abs_diff(*n) > 3) {
            return false;
        }

        prev = *n;
    }
    true
}

fn is_safe_part2(level: &Level, error: bool) -> bool {
    let mut prev = level[0];
    let increasing = level[0] < level[1];
    for i in 1..level.len() {
        let n = &level[i];
        let mut errored = false;
        if increasing && prev > *n {
            errored = true;
        }
        if !increasing && prev < *n {
            errored = true;
        }

        if (prev == *n) || (prev.abs_diff(*n) > 3) {
            errored = true;
        }

        if errored && error {
            return false;
        } else if errored {
            let mut c1 = level.clone();
            let mut c2 = level.clone();
            let mut c3 = level.clone();
            c1.remove(i - 1);
            c2.remove(i);
            if i > 1 {
                c3.remove(i - 2);
            }
            return is_safe_part2(&c1, true)
                || is_safe_part2(&c2, true)
                || is_safe_part2(&c3, true);
        }

        prev = *n;
    }
    true
}

fn part_one(input: &str) -> Option<usize> {
    let levels = get_levels(input);
    let safe = levels.iter().filter(|x| is_safe(x)).count();
    Some(safe)
}

fn part_two(input: &str) -> Option<usize> {
    let levels = get_levels(input);
    let safe = levels.iter().filter(|x| is_safe_part2(x, false)).count();
    Some(safe)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 02);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 02);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 02);
        assert_eq!(part_two(&input), Some(4));
    }
}
