use std::collections::HashMap;

use aoc2020::utils;

fn main() {
    let lst: Vec<String> = utils::read_list("inputs/d15.txt");
    let input: Vec<i32> = lst
        .get(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut numbers = HashMap::new();

    let mut current = 0;
    let mut next = 0;
    for n in input.iter() {
        numbers.insert(*n, current);
        current += 1;
    }

    while current < 2020 - 1 {
        let first = !numbers.contains_key(&next);
        let n = match first {
            true => 0,
            false => current - numbers.get(&next).unwrap(),
        };
        numbers.insert(next, current);
        current += 1;
        next = n;
    }

    println!("Part 1: {}", next);

    while current < 30000000 - 1 {
        let first = !numbers.contains_key(&next);
        let n = match first {
            true => 0,
            false => current - numbers.get(&next).unwrap(),
        };
        numbers.insert(next, current);
        current += 1;
        next = n;
    }
    println!("Part 2: {}", next);
}
