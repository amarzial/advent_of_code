use std::collections::HashMap;

use aoc2020::utils;

type Int = u64;
#[derive(Debug)]
enum Command {
    Mask(Int, Int, Vec<Int>),
    Mem(Int, Int),
}

use crate::Command::{Mask, Mem};

fn parseinput(line: &str) -> Command {
    if line.starts_with("mask") {
        let mut mask_or: Int = 0;
        let mut mask_and: Int = 0;
        let mut mask_float: Vec<Int> = Vec::new();
        let mut index = 0;
        for c in line.as_bytes().get(7..line.len()).unwrap() {
            mask_and <<= 1;
            mask_or <<= 1;
            index += 1;
            match *c {
                b'1' => {
                    mask_or |= 1;
                }
                b'0' => {
                    mask_and |= 1;
                }
                _ => {
                    mask_float.push(36 - index);
                }
            }
        }
        mask_and = !mask_and & (2_u64.pow(37) - 1);
        return Mask(mask_and, mask_or, mask_float);
    } else {
        let tmp = line.find(']').unwrap();
        let address: Int = line.get(4..tmp).unwrap().parse().unwrap();
        let value: Int = line.get(tmp + 4..line.len()).unwrap().parse().unwrap();
        return Mem(address, value);
    }
}

fn main() {
    let lst = utils::read_list_parse("inputs/d14.txt", parseinput);

    let mut memory: HashMap<Int, Int> = HashMap::new();
    let mut current_mask: (Int, Int, &Vec<Int>) = (2_u64.pow(37) - 1, 0, &Vec::new());
    for cmd in lst.iter() {
        match cmd {
            Mask(and, or, float) => {
                current_mask = (*and, *or, float);
            }
            Mem(addr, val) => {
                let mut v = *val;
                v &= current_mask.0;
                v |= current_mask.1;
                memory.insert(*addr, v);
            }
        }
    }

    let mut p1 = 0;
    for cell in memory.values() {
        p1 += *cell;
    }
    println!("Part 1: {}", p1);

    memory.clear();
    for cmd in lst.iter() {
        match cmd {
            Mask(and, or, float) => {
                current_mask = (*and, *or, float);
            }
            Mem(addr, val) => {
                let mut target_address = *addr;
                target_address |= current_mask.1;
                if current_mask.2.len() > 0 {
                    for i in 0..2_u64.pow(current_mask.2.len() as u32) {
                        for j in 0..current_mask.2.len() {
                            let cursor = current_mask.2.get(j).unwrap();
                            let bit = 1 << cursor;
                            if (i & (1 << j)) != 0 {
                                target_address |= bit;
                            } else {
                                target_address &= !bit;
                            }
                        }
                        memory.insert(target_address, *val);
                    }
                } else {
                    memory.insert(target_address, *val);
                }
            }
        }
    }

    let mut p2 = 0;
    for cell in memory.values() {
        p2 += *cell;
    }
    println!("Part 2: {}", p2);
}
