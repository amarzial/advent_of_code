use aoc2020::utils;

use std::collections::HashSet;

fn read_answers(iter: &mut std::slice::Iter<String>) -> Option<(usize, usize)> {
    let mut set = std::collections::HashSet::new();
    let mut set2: std::collections::HashSet<u8> = std::collections::HashSet::new();
    let mut first = true;
    loop {
        match iter.next() {
            Some(line) => {
                if line.is_empty() {
                    return Some((set.len(), set2.len()));
                }
                for chr in line.as_bytes().iter() {
                    set.insert(*chr);
                }
                let line_set: HashSet<u8> = line.as_bytes().iter().cloned().collect();
                if first {
                    set2 = set2.union(&line_set).cloned().collect();
                    first = false;
                } else {
                    set2 = set2.intersection(&line_set).cloned().collect();
                }
            }
            _ => {
                return if set.len() != 0 {
                    Some((set.len(), set2.len()))
                } else {
                    None
                }
            }
        }
    }
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or(String::from("inputs/d06.txt"));
    let list = utils::read_list::<String>(&filename);

    let mut lines = list.iter();

    let mut count = 0;
    let mut count2 = 0;
    while let Some(cnt) = read_answers(&mut lines) {
        count += cnt.0;
        count2 += cnt.1;
    }
    println!("Part 1: {}", count);
    println!("Part 2: {}", count2);
}
