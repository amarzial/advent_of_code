use std::mem::discriminant;

use json::{self, JsonValue};

fn compare(lhs: &JsonValue, rhs: &JsonValue) -> std::cmp::Ordering {
    if discriminant(lhs) != discriminant(rhs) {
        let mut a = lhs.clone();
        let mut b = rhs.clone();
        if discriminant(lhs) == discriminant(&JsonValue::Number(json::number::Number::from(0))) {
            a = JsonValue::Array(vec![lhs.clone()]);
        }
        if discriminant(rhs) == discriminant(&JsonValue::Number(json::number::Number::from(0))) {
            b = JsonValue::Array(vec![rhs.clone()]);
        }
        return compare(&a, &b);
    } else {
        if let JsonValue::Number(_) = lhs {
            return lhs.as_i32().unwrap().cmp(&rhs.as_i32().unwrap());
        } else {
            let min = lhs.len().min(rhs.len());
            for i in 0..min {
                let cmp = compare(&lhs[i], &rhs[i]);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            return lhs.len().cmp(&rhs.len());
        }
    }
}

fn part_one(input: &str) -> Option<i32> {
    let tokens: Vec<Vec<JsonValue>> = input
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter(|s| s.len() > 0)
                .map(|l| json::parse(l).unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;
    let mut current = 1;
    for pair in tokens.iter() {
        total += match compare(&pair[0], &pair[1]) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => current,
            std::cmp::Ordering::Greater => 0,
        };
        current += 1;
    }
    Some(total)
}

fn part_two(input: &str) -> Option<i32> {
    let mut tokens: Vec<JsonValue> = input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| json::parse(s).unwrap())
        .collect();

    let one = json::parse("[[2]]").unwrap();
    let two = json::parse("[[6]]").unwrap();
    tokens.push(one.clone());
    tokens.push(two.clone());

    tokens.sort_by(compare);

    let mut total = 1;
    let mut current = 1;
    for line in tokens.iter() {
        if (compare(line, &one) == std::cmp::Ordering::Equal)
            || (compare(line, &two) == std::cmp::Ordering::Equal)
        {
            total *= current;
        }
        current += 1;
    }

    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 13);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
