use std::collections::HashMap;

use itertools::Itertools;

type Int = i64;

fn move_elem(e: &(Int, Int), list: &mut Vec<(Int, Int)>) {
    let mut idx = list.iter().find_position(|x| *x == e).unwrap().0 as Int;
    let e = list.remove(idx as usize);
    idx += e.0;
    idx %= list.len() as Int;
    if idx < 0 {
        idx += list.len() as Int;
    }
    list.insert(idx as usize, e);
}

fn part_one(input: &str) -> Option<Int> {
    let numbers: Vec<Int> = aoc::utils::read_list(input);

    let mut nth: HashMap<Int, Int> = HashMap::new();
    let mut list = Vec::new();
    list.reserve(numbers.len());
    for l in numbers.iter() {
        let n = nth.get(l).unwrap_or(&0).clone();
        nth.insert(*l, n + 1);

        list.push((*l, n));
    }

    let order = list.clone();

    for e in order.iter() {
        move_elem(e, &mut list);
    }

    let mut result = 0;
    let mut start = list.iter().find_position(|x| (**x).0 == 0).unwrap().0;
    start = (start + 1000) % list.len();
    result += list[start].0;
    start = (start + 1000) % list.len();
    result += list[start].0;
    start = (start + 1000) % list.len();
    result += list[start].0;

    Some(result)
}

fn part_two(input: &str) -> Option<Int> {
    let numbers: Vec<Int> = aoc::utils::read_list(input);

    let decryption_key = 811589153;

    let mut nth: HashMap<Int, Int> = HashMap::new();
    let mut list = Vec::new();
    list.reserve(numbers.len());
    for l in numbers.iter() {
        let num = l * decryption_key;
        let n = nth.get(&num).unwrap_or(&0).clone();
        nth.insert(num, n + 1);

        list.push((num, n));
    }

    let order = list.clone();
    for _ in 0..10 {
        for e in order.iter() {
            move_elem(e, &mut list);
        }
    }

    let mut result = 0;
    let mut start = list.iter().find_position(|x| (**x) == (0, 0)).unwrap().0;
    start = (start + 1000) % list.len();
    result += list[start].0;
    start = (start + 1000) % list.len();
    result += list[start].0;
    start = (start + 1000) % list.len();
    result += list[start].0;

    Some(result)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 20);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
