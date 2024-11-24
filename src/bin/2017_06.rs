use std::collections::HashMap;

use aoc::utils;
use itertools::Itertools;

fn redist(vec: &mut Vec<i32>) {
    let mut biggest = 0;
    let mut max = 0;
    for i in 0..vec.len() {
        if vec[i] > max {
            max = vec[i];
            biggest = i;
        }
    }

    vec[biggest] = 0;

    while max > 0 {
        biggest = (biggest + 1) % vec.len();
        vec[biggest] = vec[biggest] + 1;
        max -= 1;
    }
}

fn hash(vec: &Vec<i32>) -> String {
    vec.iter().map(|v| v.to_string()).join(":")
}

fn run(input: &str) -> (i32, i32) {
    let mut res = utils::read_line::<i32>(input, "\t");
    let mut hashes = HashMap::new();
    hashes.insert(hash(&res), 0);
    let mut count = 0;

    let loop_size;
    loop {
        count += 1;
        redist(&mut res);

        let h = hash(&res);
        if hashes.contains_key(&h) {
            loop_size = count - hashes.get(&h).unwrap();
            break;
        }
        hashes.insert(h, count);
    }
    (count, loop_size)
}

fn part_one(input: &str) -> Option<i32> {
    let res = run(input);
    Some(res.0)
}

fn part_two(input: &str) -> Option<i32> {
    let res = run(input);
    Some(res.1)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2017, 06);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2017, 06);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2017, 06);
        assert_eq!(part_two(&input), Some(4));
    }
}
