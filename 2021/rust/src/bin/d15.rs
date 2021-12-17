use aoc::utils;
extern crate pathfinding;

use pathfinding::prelude::astar;

struct Grid {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    mult: usize,
}

impl Grid {
    fn bounds_check(&self, pos: (i32, i32)) -> bool {
        (pos.1 >= 0) && (pos.1 < self.height as i32) && (pos.0 >= 0) && (pos.0 < self.width as i32)
    }
    fn get(&self, mut pos: (usize, usize)) -> Option<u8> {
        let increment = pos.0 / (self.width / self.mult) + pos.1 / (self.height / self.mult);
        pos.0 = pos.0 % (self.width / self.mult);
        pos.1 = pos.1 % (self.height / self.mult);
        match self.data.get(pos.1) {
            Some(v) => match v.get(pos.0) {
                Some(val) => {
                    let out = ((*val) as usize + increment) as u8;
                    Some(if out < 10 { out } else { out % 10 + 1 })
                }
                None => None,
            },
            None => None,
        }
    }
}

fn run(input: &Grid) -> i32 {
    let successors = |e: &(i32, i32)| -> Vec<((i32, i32), i32)> {
        let p = *e;
        let s = vec![
            (p.0 - 1, p.1),
            (p.0, p.1 - 1),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
        ]
        .into_iter()
        .filter(|e| {
            return input.bounds_check((e.0, e.1));
        })
        .map(|e: (i32, i32)| {
            let weight = input.get((e.0 as usize, e.1 as usize)).unwrap_or(u8::MAX);
            return (e, weight as i32);
        })
        .collect();
        s
    };

    let start = (0, 0);
    let target = (input.width as i32 - 1, input.height as i32 - 1);
    let path = astar(
        &start,
        successors,
        |f| (f.0 - input.width as i32) + (f.1 - input.height as i32),
        |v| *v == target,
    );

    let res = path.unwrap();
    return res.1;
}

fn main() {
    let mut input: Grid = Grid {
        data: utils::read_list_parse(&utils::get_input(), |f| {
            return f.chars().map(|e| e.to_digit(10).unwrap() as u8).collect();
        }),
        width: 0,
        height: 0,
        mult: 1,
    };

    input.height = input.data.len();
    input.width = input.data[0].len();

    println!("Part 1: {}", run(&input));
    input.mult = 5;
    input.width *= input.mult;
    input.height *= input.mult;
    // input.print_path(&Vec::new());
    println!("Part 2: {}", run(&input));
}
