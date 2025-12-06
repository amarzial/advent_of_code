fn read_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect()
}

fn part_one(input: &str) -> Option<String> {
    let grid = read_input(input);

    let operands = grid.last().unwrap();

    let results: i64 = operands
        .iter()
        .enumerate()
        .map(|(k, v)| -> i64 {
            let op = match *v {
                "+" => |a, b| a + b,
                "*" => |a, b| a * b,
                _ => panic!("Unknown operand"),
            };
            let start = match *v {
                "+" => 0,
                "*" => 1,
                _ => panic!("Unknown operand"),
            };
            let res = grid
                .iter()
                .take(grid.len() - 1)
                .fold(start, |acc, row| op(acc, row[k].parse::<i64>().unwrap()));
            res
        })
        .sum();

    Some(results.to_string())
}

fn part_two(input: &str) -> Option<String> {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut vals = vec![];
    let mut count = rows.last().unwrap().iter().filter(|c| **c != ' ').count() - 1;
    for i in (0..rows[0].len()).rev() {
        let mut val = 0;
        let mut valid = false;
        for row in &rows[0..rows.len() - 1] {
            let chr = row[i];
            match chr.to_digit(10) {
                Some(d) => {
                    val = val * 10 + d as i64;
                    valid = true;
                }
                None => {}
            }
        }
        if !valid {
            count -= 1;
        } else {
            vals.push((count, val));
        }
    }

    let mut total = 0;
    let s = String::from_iter(rows.last().unwrap());
    for (i, op) in s.split_ascii_whitespace().enumerate() {
        let start = match op {
            "*" => 1,
            "+" => 0,
            _ => 0,
        };
        let operation = match op {
            "*" => |a, b| a * b,
            "+" => |a, b| a + b,
            _ => panic!(),
        };

        total += vals
            .iter()
            .filter(|v| v.0 == i)
            .map(|v| v.1)
            .fold(start, |acc, val| operation(acc, val));
    }
    Some(total.to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2025, 06);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2025, 06);
        assert_eq!(part_one(&input), Some("4277556".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2025, 06);
        assert_eq!(part_two(&input), Some("3263827".to_string()));
    }
}
