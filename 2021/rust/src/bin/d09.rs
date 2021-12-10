use std::{collections::HashMap, hash::Hash};

use aoc2021::utils;

type Grid = Vec<Vec<u32>>;

fn part_one(g: &Grid) -> u32 {
    let mut sum = 0;
    let positions = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    for y in 0..g.len() {
        let row = &g[y];
        for x in 0..row.len() {
            let mut lowest = 9;
            for p in positions.iter() {
                match g.get((y as i32 + p.0) as usize) {
                    Some(v) => match v.get((x as i32 + p.1) as usize) {
                        Some(n) => {
                            lowest = lowest.min(*n);
                        }
                        None => {}
                    },
                    None => {}
                }
            }
            let current = g[y][x];
            sum += if current < lowest { 1 + current } else { 0 };
        }
    }
    return sum;
}

fn main() {
    let input: Grid = utils::read_list_parse(&utils::get_input(), |x| {
        x.chars().map(|c| c.to_digit(10).unwrap()).collect()
    });

    let p1 = part_one(&input);
    println!("Part 1: {}", p1);
}
