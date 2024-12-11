use std::collections::HashMap;

type Int = u64;

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|f| f.to_string())
        .collect()
}

fn add_stones(stones: &mut HashMap<String, Int>, stone: &str, count: Int) {
    stones.insert(stone.to_string(), stones.get(stone).unwrap_or(&0) + count);
}

fn part_one(input: &str) -> Option<Int> {
    let line = parse_input(input);
    let mut stones = HashMap::new();
    let mut buffer = HashMap::new();

    for stone in line.iter() {
        add_stones(&mut stones, stone, 1);
    }

    for _i in 0..25 {
        buffer.clear();
        for info in stones.iter() {
            let stone = info.0;
            if stone == "0" {
                add_stones(&mut buffer, "1", *info.1);
            } else if stone.len() % 2 == 0 {
                let sp = stone.split_at(stone.len() / 2);
                add_stones(&mut buffer, sp.0, *info.1);
                add_stones(
                    &mut buffer,
                    sp.1.to_string()
                        .parse::<Int>()
                        .unwrap()
                        .to_string()
                        .as_str(),
                    *info.1,
                );
            } else {
                let s = (stone.parse::<Int>().unwrap() * 2024).to_string();
                add_stones(&mut buffer, &s, *info.1);
            }
        }
        std::mem::swap(&mut stones, &mut buffer);
    }

    Some(stones.iter().map(|f| f.1).sum())
}

fn part_two(input: &str) -> Option<Int> {
    let line = parse_input(input);
    let mut stones = HashMap::new();
    let mut buffer = HashMap::new();

    for stone in line.iter() {
        add_stones(&mut stones, stone, 1);
    }

    for _i in 0..75 {
        buffer.clear();
        for info in stones.iter() {
            let stone = info.0;
            if stone == "0" {
                add_stones(&mut buffer, "1", *info.1);
            } else if stone.len() % 2 == 0 {
                let sp = stone.split_at(stone.len() / 2);
                add_stones(&mut buffer, sp.0, *info.1);
                add_stones(
                    &mut buffer,
                    sp.1.to_string()
                        .parse::<Int>()
                        .unwrap()
                        .to_string()
                        .as_str(),
                    *info.1,
                );
            } else {
                let s = (stone.parse::<Int>().unwrap() * 2024).to_string();
                add_stones(&mut buffer, &s, *info.1);
            }
        }
        std::mem::swap(&mut stones, &mut buffer);
    }

    Some(stones.iter().map(|f| f.1).sum())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 11);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 11);
        assert_eq!(part_one(&input), Some(55312));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 11);
        assert_eq!(part_two(&input), None);
    }
}
