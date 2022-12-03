use std::collections::HashSet;

fn parse_bag(line: &str) -> (HashSet<char>, HashSet<char>) {
    let mut left = HashSet::new();
    let mut right = HashSet::new();
    let mut chars = line.chars();
    for i in 0..line.len() {
        let c = chars.next().unwrap();
        if i < line.len() / 2 {
            left.insert(c);
        } else {
            right.insert(c);
        }
    }
    (left, right)
}

fn parse_bag_2(line: &str) -> HashSet<char> {
    line.chars().collect()
}

fn char_value(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        ((c as u8 - 'a' as u8) + 1) as i32
    } else {
        ((c as u8 - 'A' as u8) + 27) as i32
    }
}

fn part_one(input: &str) -> Option<String> {
    let bags = aoc::utils::read_list_parse(input, parse_bag);
    let mut value = 0;
    for b in bags.iter() {
        let elem = b.0.intersection(&b.1);
        for e in elem.into_iter() {
            value += char_value(*e);
        }
    }
    Some(value.to_string())
}

fn part_two(input: &str) -> Option<String> {
    let mut bags = aoc::utils::read_list_parse(input, parse_bag_2);

    let mut value = 0;
    for group in bags.chunks_mut(3) {
        let mut g = group.iter();
        let mut s1 = g.next().unwrap().clone();
        let s2 = g.next().unwrap();
        let s3 = g.next().unwrap();
        s1.retain(|c| s2.contains(c) && s3.contains(c));
        for s in s1.iter() {
            value += char_value(*s);
        }
    }

    Some(value.to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 03);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 03);
        assert_eq!(part_one(&input), Some(157.to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 03);
        assert_eq!(part_two(&input), Some(70.to_string()));
    }
}
