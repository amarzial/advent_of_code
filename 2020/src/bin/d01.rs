// use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use aoc2020::utils;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or(String::from("inputs/d01.txt"));
    let list = utils::read_list(&filename);

    //part 1
    let set: HashSet<u32> = HashSet::from_iter(list.clone().into_iter());
    for i in set.iter() {
        if set.contains(&(2020 - i)) {
            println!("Part 1: {}", i * (2020 - i));
            break;
        }
    }

    //part2
    let mut set: HashMap<u32, (u32, u32)> = HashMap::new();
    for i in list.iter() {
        for j in list.iter() {
            if i != j {
                set.insert(i + j, (*i, *j));
            }
        }
    }

    for i in list.iter() {
        if set.contains_key(&(2020 - i)) {
            let others = set.get(&(2020 - i)).unwrap();
            println!("Part 2: {}", (2020 - i) * others.0 * others.1);
            break;
        }
    }
}
