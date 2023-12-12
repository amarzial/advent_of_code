use std::collections::HashMap;

use itertools::Itertools;

type Int = i64;

fn count_solutions(
    cache: &mut HashMap<(Int, Int, Int), Int>,
    input: &[u8],
    groups: &[Int],
    current_len: Int,
) -> Int {
    if groups.len() == 0 && current_len != 0 {
        return 0;
    }
    if input.len() == 0 {
        return if groups.len() == 1 && current_len == groups[0] {
            1
        } else if groups.len() == 0 && current_len == 0 {
            1
        } else {
            0
        };
    }

    let cache_key = (input.len() as Int, groups.len() as Int, current_len as Int);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let solutions = match (input[0], current_len) {
        (b'#', _) => count_solutions(cache, &input[1..], groups, current_len + 1),
        (b'.', 0) => count_solutions(cache, &input[1..], groups, 0),
        (b'.', x) if x != groups[0] => 0,
        (b'.', _) => count_solutions(cache, &input[1..], &groups[1..], 0),
        (b'?', 0) => {
            count_solutions(cache, &input[1..], groups, current_len + 1)
                + count_solutions(cache, &input[1..], groups, 0)
        }
        (b'?', _) => {
            let mut s = count_solutions(cache, &input[1..], groups, current_len + 1);
            if current_len == groups[0] {
                s += count_solutions(cache, &input[1..], &groups[1..], 0);
            }
            s
        }
        _ => unreachable!("{}: {}", input[0] as char, current_len),
    };
    cache.insert(cache_key, solutions);
    solutions
}

fn part_one(input: &str) -> Option<Int> {
    let mut cache = HashMap::new();
    let i: Int = input
        .lines()
        .map(|l| {
            let (springs, sec) = l.split_once(' ').unwrap();
            let groups = sec
                .split(',')
                .map(|v| v.parse::<Int>().unwrap())
                .collect_vec();

            cache.clear();
            let res = count_solutions(&mut cache, springs.as_bytes(), &groups, 0);
            res
        })
        .sum();

    Some(i)
}

fn part_two(input: &str) -> Option<Int> {
    let mut cache = HashMap::new();
    let i: Int = input
        .lines()
        .map(|l| {
            let (springs, sec) = l.split_once(' ').unwrap();
            let mut groups = sec
                .split(',')
                .map(|v| v.parse::<Int>().unwrap())
                .collect_vec();

            let new_springs = (0..5).map(|_| springs).join("?");
            groups = groups.repeat(5);
            cache.clear();
            let res = count_solutions(&mut cache, new_springs.as_bytes(), &groups, 0);

            res
        })
        .sum();

    Some(i)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 12);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 12);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 12);
        assert_eq!(part_two(&input), Some(525152));
    }
}
