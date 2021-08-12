use aoc2020::utils;

fn locate(position: &str) -> u32 {
    let mut row: u32 = 0;
    for c in 0..7 {
        let chr = position.as_bytes().get(c).unwrap();
        row <<= 1;
        row |= if *chr == b'B' { 1 } else { 0 }
    }
    let mut col: u32 = 0;
    for c in 7..10 {
        let chr = position.as_bytes().get(c).unwrap();
        col <<= 1;
        col |= if *chr == b'R' { 1 } else { 0 }
    }

    return row * 8 + col;
}

use std::collections::BTreeSet;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or(String::from("inputs/d05.txt"));
    let list = utils::read_list::<String>(&filename);

    let mut max = 0;
    let mut search = BTreeSet::new();
    for r in list.iter() {
        let val = locate(r);
        max = if max < val { val } else { max };
        search.insert(val);
    }
    println!("Part 1: {}", max);
    let mut last = search.iter().next().unwrap();
    for e in search.iter() {
        if *e > last + 1 {
            println!("Part 2: {}", *e - 1);
            break;
        }
        last = e;
    }
}
