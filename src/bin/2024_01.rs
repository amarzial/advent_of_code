use std::{collections::HashMap, iter::zip};

fn parse(input: &str) -> Vec<(i32, i32)> {
    let list: Vec<(i32, i32)> = aoc::utils::read_list_parse(input, |f| {
        let mut sp = f.split_whitespace();
        (
            sp.next().unwrap().parse().unwrap(),
            sp.next().unwrap().parse().unwrap(),
        )
    });

    return list;
}

fn part_one(input: &str) -> Option<i32> {
    let list = parse(input);
    let mut l1: Vec<i32> = list.iter().map(|x| x.0).collect();
    let mut l2: Vec<i32> = list.iter().map(|x| x.1).collect();

    l1.sort();
    l2.sort();

    let out = zip(l1, l2).map(|f| f.0.abs_diff(f.1)).sum::<u32>() as i32;
    Some(out)
}

fn part_two(input: &str) -> Option<i64> {
    let list = parse(input);
    let l1: Vec<i32> = list.iter().map(|x| x.0).collect();
    let l2: Vec<i32> = list.iter().map(|x| x.1).collect();

    let mut cache = HashMap::new();

    let mut total: i64 = 0;
    for n in l1 {
        if cache.contains_key(&n) {
            total += *cache.get(&n).unwrap() as i64;
            continue;
        }
        let mut count = 0;
        for m in l2.iter() {
            if *m == n {
                count += 1;
            }
        }
        let value = count * n;
        cache.insert(n, value);
        total += value as i64;
    }
    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 01);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 01);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 01);
        assert_eq!(part_two(&input), Some(31));
    }
}
