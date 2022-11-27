use crate::day;

use std::collections::HashMap;

fn do_run(fishes: &mut HashMap<i32, u64>) {
    for i in 0..=8 {
        fishes.insert(i - 1, fishes[&i]);
        fishes.insert(i, 0);
    }
    *fishes.entry(6).or_insert(0) += fishes[&-1];
    *fishes.entry(8).or_insert(0) += fishes[&-1];
    fishes.insert(-1, 0);
}

pub fn run(d: &mut day::Day) -> bool {
    let input = d.input.clone();
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
        do_run(&mut counters);
    }

    d.set_part_1(counters.values().sum::<u64>().to_string());

    for _i in 80..256 {
        do_run(&mut counters);
    }
    d.set_part_2(counters.values().sum::<u64>().to_string());
    true
}
