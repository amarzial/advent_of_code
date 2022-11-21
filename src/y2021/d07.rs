use crate::day;
use crate::utils;

fn calc_fuel(crabs: &Vec<i32>, target: i32) -> i32 {
    let mut fuel = 0;
    for c in crabs.iter() {
        fuel += i32::abs(*c - target);
    }
    return fuel;
}

fn calc_fuel2(crabs: &Vec<i32>, target: i32) -> i32 {
    let mut fuel = 0;
    for c in crabs.iter() {
        let distance = i32::abs(*c - target);
        for i in 1..=distance {
            fuel += i;
        }
    }
    return fuel;
}

pub fn run(d: &mut day::Day) -> bool {
    let mut input: Vec<i32> = utils::read_line(&d.input, ",");
    input.sort();

    let median = input[input.len() / 2 - 1];
    let p1 = calc_fuel(&input, median);
    d.set_part_1(p1.to_string());

    let avg = input.iter().sum::<i32>() / input.len() as i32;
    let p2 = calc_fuel2(&input, avg);
    d.set_part_2(p2.to_string());
    true
}
