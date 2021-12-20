use std::collections::{HashMap, HashSet};

use aoc::utils;

type GridData = HashMap<(i32, i32), bool>;
type Mapping = HashMap<usize, char>;
struct Grid {
    main: GridData,
    back: GridData,
    light: bool,
}

impl Grid {
    fn new<I>(input: I) -> Grid
    where
        I: IntoIterator<Item = String>,
    {
        let mut grid = HashMap::new();
        let mut y = 0;
        for line in input {
            let mut x = 0;
            for c in line.chars() {
                if c == '#' {
                    grid.insert((x, y), true);
                }
                x += 1;
            }
            y += 1;
        }
        Grid {
            main: grid,
            back: GridData::new(),
            light: true,
        }
    }
    fn run(&mut self, mapping: &Mapping) {
        let mut next = HashSet::new();

        let area = |p: (i32, i32)| -> Vec<(i32, i32)> {
            vec![
                (p.0 - 1, p.1 - 1),
                (p.0, p.1 - 1),
                (p.0 + 1, p.1 - 1),
                (p.0 - 1, p.1),
                (p.0, p.1),
                (p.0 + 1, p.1),
                (p.0 - 1, p.1 + 1),
                (p.0, p.1 + 1),
                (p.0 + 1, p.1 + 1),
            ]
        };

        let base;
        if self.light {
            base = &mut self.main;
        } else {
            base = &mut self.back;
        }

        for pixel in base.keys() {
            for p in area(*pixel) {
                next.insert(p);
            }
        }

        let mut new_light: GridData = GridData::new();
        let mut new_dark: GridData = GridData::new();
        for pixel in next.iter() {
            let mut val = 0;
            for p in area(*pixel) {
                val <<= 1;
                if base.contains_key(&p) == self.light {
                    val |= 1
                }
            }

            if mapping[&val] == '#' {
                new_light.insert(*pixel, true);
            } else {
                new_dark.insert(*pixel, true);
            }
        }
        if mapping[&0] == '#' {
            self.light = !self.light;
        } else {
        }
        std::mem::swap(&mut self.main, &mut new_light);
        std::mem::swap(&mut self.back, &mut new_dark);
    }
}
fn main() {
    let mut input = utils::read_list::<String>(&utils::get_input()).into_iter();
    let mapping: Mapping = input.next().unwrap().chars().enumerate().collect();
    input.next();
    let mut grid = Grid::new(input);

    for _i in 0..2 {
        grid.run(&mapping);
    }
    println!("Part 1: {}", grid.main.len());
    for _i in 2..50 {
        grid.run(&mapping);
    }
    println!("Part 2: {}", grid.main.len());
}
