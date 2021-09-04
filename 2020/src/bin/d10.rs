use aoc2020::utils;
use std::cell::RefCell;
use std::collections::HashMap;

fn combinazioni(
    list: &Vec<u32>,
    index: usize,
    cache: &RefCell<HashMap<usize, u64>>,
    level: usize,
) -> u64 {
    // println!("{0:1$}{2}", "", level, list.get(index).unwrap());
    if index == list.len() - 1 {
        return 1;
    }

    let mut result = 0;
    let mut c = 1;
    while (index < list.len())
        && (index + c < list.len())
        && list.get(index + c).unwrap() - list.get(index).unwrap() <= 3
    {
        let combs;
        if (*cache).borrow().get(&(index + c)).is_some() {
            combs = *(*cache).borrow().get(&(index + c)).unwrap();
        } else {
            combs = combinazioni(&list, index + c, cache, level + 1);
            cache.borrow_mut().insert(index + c, combs);
        }
        result += combs;
        c += 1;
    }
    result
}

fn main() {
    let mut adapters = utils::read_list::<u32>("inputs/d10.txt");
    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let cache = RefCell::new(HashMap::new());

    let mut diff1 = 0;
    let mut diff3 = 0;
    let mut last = 0;
    let mut iter = adapters.iter();
    iter.next();
    for a in iter {
        if *a - last == 1 {
            diff1 += 1;
        } else {
            diff3 += 1;
        }
        last = *a;
    }
    println!("Part 1: {}", diff1 * diff3);

    println!("Part 2: {}", combinazioni(&adapters, 0, &cache, 0));
}
