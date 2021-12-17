use aoc::utils;

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

fn main() {
    let mut input: Vec<i32> = utils::read_line(&utils::get_input(), ",");
    input.sort();

    let median = input[input.len() / 2 - 1];
    let p1 = calc_fuel(&input, median);
    println!("Part 1: {}", p1);

    let avg = input.iter().sum::<i32>() / input.len() as i32;
    let p2 = calc_fuel2(&input, avg);
    println!("Part 2: {}", p2);
}
