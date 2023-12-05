use std::collections::HashMap;

type Int = u64;
type Mapping = Vec<(Int, Int, Int)>;

fn parser(input: &str) -> (Vec<Int>, Vec<Mapping>) {
    let lines: Vec<String> = aoc::utils::read_list(input);
    let mut it = lines.iter();

    let seeds: Vec<Int> = it
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|v| v.parse::<Int>().unwrap())
        .collect();

    it.next();

    let mut maps: Vec<Mapping> = vec![];
    loop {
        let head = it.next();
        if head.is_none() {
            break;
        }
        let mut mapping = Mapping::new();
        loop {
            match it.next() {
                Some(line) => {
                    if line == "" {
                        break;
                    }
                    let values: Vec<Int> =
                        line.split(' ').map(|v| v.parse::<Int>().unwrap()).collect();
                    mapping.push((values[0], values[1], values[2]));
                }
                None => {
                    break;
                }
            }
        }
        maps.push(mapping);
    }

    (seeds, maps)
}

fn lookup(value: Int, map: &Mapping) -> Int {
    for m in map {
        if (value >= m.1) && (value < m.1 + m.2) {
            return m.0 + (value - m.1);
        }
    }
    return value;
}

fn location(seed: Int, maps: &Vec<Mapping>) -> Int {
    let mut current = seed;
    for map in maps {
        current = lookup(current, map);
    }
    return current;
}

fn part_one(input: &str) -> Option<Int> {
    let (seeds, maps) = parser(input);

    let min = seeds.iter().map(|v| location(*v, &maps)).min().unwrap();
    Some(min)
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 05);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 05);
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 05);
        assert_eq!(part_two(&input), None);
    }
}
