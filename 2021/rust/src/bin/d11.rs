use std::collections::{HashSet, VecDeque};

use aoc::utils;

type Grid = Vec<Vec<u32>>;

fn step(grid: &mut Grid) -> u32 {
    let mut flashes = 0;
    let mut queue = VecDeque::new();
    for y in 0..10 {
        for x in 0..10 {
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                queue.push_back(y * 10 + x);
            }
        }
    }

    let positions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut flashed = HashSet::new();
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        if flashed.contains(&current) {
            continue;
        }
        let x = current % 10;
        let y = current / 10;
        if grid[y][x] > 9 {
            flashed.insert(current);
            flashes += 1;
            //flash
            for pos in positions.iter() {
                let newx = x as i32 + pos.0;
                let newy = y as i32 + pos.1;
                if (newx >= 0) && (newx < 10) && (newy >= 0) && (newy < 10) {
                    grid[newy as usize][newx as usize] += 1;
                    queue.push_back((y as i32 + pos.1) as usize * 10 + (x as i32 + pos.0) as usize);
                }
            }
        }
    }
    for v in flashed.iter() {
        grid[*v / 10][*v % 10] = 0;
    }
    return flashes;
}

fn main() {
    let mut input: Grid = utils::read_list_parse(&utils::get_input(), |x| {
        x.chars().map(|c| c.to_digit(10).unwrap()).collect()
    });

    let mut p1 = 0;
    let mut p2 = None;
    for i in 0..100 {
        let flashes = step(&mut input);
        p1 += flashes;
        if (flashes == 100) && p2.is_none() {
            p2 = Some(i + 1);
        }
    }
    let mut i = 100;
    while p2.is_none() {
        let flashes = step(&mut input);
        if flashes == 100 {
            p2 = Some(i + 1);
        }
        i += 1;
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2.unwrap());
}
