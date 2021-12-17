use std::collections::HashMap;

use aoc::utils;

fn run(fishes: &mut HashMap<i32, u64>) {
    for i in 0..=8 {
        fishes.insert(i - 1, fishes[&i]);
        fishes.insert(i, 0);
    }
    *fishes.entry(6).or_insert(0) += fishes[&-1];
    *fishes.entry(8).or_insert(0) += fishes[&-1];
    fishes.insert(-1, 0);
}

fn main() {
    let input = std::fs::read_to_string(&utils::get_input()).unwrap();
    let fishes: Vec<i32> = input[0..input.len() - 1]
        .split(",")
        .map(|x| return x.parse().unwrap())
        .collect();

    let mut counters: HashMap<i32, u64> = HashMap::new();
    for i in -1..=8 {
        counters.insert(i, 0);
    }
    for f in fishes.iter() {
        *counters.entry(*f).or_insert(0) += 1;
    }
    for _i in 0..80 {
        run(&mut counters);
    }

    println!("Part 1: {}", counters.values().sum::<u64>());

    for _i in 80..256 {
        run(&mut counters);
    }

    println!("Part 2: {}", counters.values().sum::<u64>());
}
