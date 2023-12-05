use itertools::Itertools;

fn get_number(s: &str) -> i32 {
    let first = s.chars().find(|c| c.is_digit(10)).unwrap();
    let last = s.chars().rev().find(|c| c.is_digit(10)).unwrap();
    let mut res = String::new();

    res.push(first);
    res.push(last);
    res.parse::<i32>().unwrap()
}

fn get_number2(s: &str) -> i32 {
    let text = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_digit_idx = s
        .chars()
        .find_position(|c| c.is_digit(10))
        .unwrap_or((usize::MAX, '0'));
    let last_digit_idx = s
        .chars()
        .rev()
        .find_position(|c| c.is_digit(10))
        .unwrap_or((0, '0'));

    let mut first_idx = first_digit_idx.0;
    let mut last_idx = if last_digit_idx.1 != '0' {
        s.len() - 1 - last_digit_idx.0
    } else {
        0
    };
    let mut first_val = first_digit_idx.1.to_digit(10).unwrap();
    let mut last_val = last_digit_idx.1.to_digit(10).unwrap();

    for i in 0..text.len() {
        let t = text[i];
        match s.find(t) {
            Some(idx) => {
                if idx < first_idx {
                    first_idx = idx;
                    first_val = (i as u32) + 1;
                }
            }
            None => {}
        }
        match s.rfind(t) {
            Some(idx) => {
                if idx > last_idx {
                    last_idx = idx;
                    last_val = (i as u32) + 1;
                }
            }
            None => {}
        }
    }

    (first_val * 10 + last_val) as i32
}

fn part_one(input: &str) -> Option<i32> {
    let list = aoc::utils::read_list::<String>(input);

    let sums: Vec<i32> = list.iter().map(|s| get_number(s)).collect();

    Some(sums.iter().sum())
}

fn part_two(input: &str) -> Option<i32> {
    let list = aoc::utils::read_list::<String>(input);

    let sums: Vec<i32> = list.iter().map(|s| get_number2(s)).collect();

    Some(sums.iter().sum())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 01);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 01);
        assert_eq!(part_one(&input), Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 01);
        assert_eq!(part_two(&input), Some(281));
    }
}
