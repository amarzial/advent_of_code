use std::collections::HashSet;

fn find_marker(input: &str, size: usize) -> Option<i32> {
    let mut chars: HashSet<u8> = HashSet::new();
    chars.reserve(size);

    let mut position: i32 = 0;
    for w in input.as_bytes().windows(size) {
        chars.clear();
        for c in w {
            match chars.contains(c) {
                true => {
                    continue;
                }
                false => {
                    chars.insert(*c);
                }
            }
        }
        if chars.len() == size {
            return Some(position + size as i32);
        }
        position += 1;
    }
    None
}

fn part_one(input: &str) -> Option<i32> {
    find_marker(input, 4)
}

fn part_two(input: &str) -> Option<i32> {
    find_marker(input, 14)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 06);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 06);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 06);
        assert_eq!(part_two(&input), Some(26));
    }
}
