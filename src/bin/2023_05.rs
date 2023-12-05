use std::vec;

type Int = u64;
type Mapping = Vec<(Int, Int, Int)>;
type Range = (Int, Int);

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
        mapping.sort_by(|a, b| Ord::cmp(&a.1, &b.1));
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

fn lookup2(mut range: Range, map: &Mapping) -> Vec<Range> {
    let mut result = vec![];
    for r in map {
        if range.1 == 0 {
            break;
        }
        if range.0 + range.1 > r.1 && r.1 + r.2 > range.0 {
            if range.0 < r.1 {
                let diff = r.1 - range.0;
                result.push((range.0, diff));
                range.0 += diff;
                range.1 -= diff;
            }

            let len = Int::min(range.0 + range.1, r.1 + r.2) - range.0;
            result.push((range.0 + r.0 - r.1, len));
            range.0 += len;
            range.1 -= len;
        } else {
            if r.1 > range.0 {
                break;
            }
        }
    }
    if range.1 > 0 {
        result.push(range);
    }
    return result;
}

fn location2(seed_range: Range, maps: &Vec<Mapping>) -> Vec<Range> {
    let mut current = vec![seed_range];
    for map in maps {
        let mut results = vec![];
        for range in current {
            results.extend(lookup2(range, map));
        }
        current = results;
    }
    return current;
}

fn part_two(input: &str) -> Option<Int> {
    let (seeds, maps) = parser(input);

    let mut results = vec![];

    for range in seeds.chunks(2) {
        results.extend(location2((range[0], range[1]), &maps));
    }

    Some(results.iter().map(|r| r.0).min().unwrap())
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
        assert_eq!(part_two(&input), Some(46));
    }
}
