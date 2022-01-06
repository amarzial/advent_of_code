struct Grid {
    width: usize,
    height: usize,
    right: Vec<usize>,
    bottom: Vec<usize>,
    grid: Vec<char>,
    right_next: Vec<usize>,
    bottom_next: Vec<usize>,
    right_prev: Vec<usize>,
    bottom_prev: Vec<usize>,
}

impl Grid {
    fn new(input: &Vec<String>) -> Grid {
        let height = input.len();
        let width = input[0].len();
        let mut g = Grid {
            width,
            height,
            right: Vec::new(),
            bottom: Vec::new(),
            grid: Vec::new(),
            right_next: Vec::new(),
            bottom_next: Vec::new(),
            right_prev: Vec::new(),
            bottom_prev: Vec::new(),
        };
        g.grid.resize(width * height, '.');
        g.right.reserve(width * height);
        g.bottom.reserve(width * height);
        g.right_next.reserve(width * height);
        g.bottom_next.reserve(width * height);

        let mut y = 0;
        for line in input {
            let mut x = 0;
            for char in line.chars() {
                let pos = y * g.width + x;
                g.grid[pos] = char;
                match char {
                    '>' => {
                        g.right.push(pos);
                    }
                    'v' => {
                        g.bottom.push(pos);
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }

        g.bottom.sort_by(|a, b| {
            let cmp = usize::cmp(&(a % width), &(b % width));
            match cmp {
                std::cmp::Ordering::Equal => usize::cmp(&(a / width), &(b / width)),
                _ => cmp,
            }
        });
        g
    }

    fn step(&mut self) -> bool {
        self.right_next.clear();
        self.bottom_next.clear();
        self.right_prev.clear();
        self.bottom_prev.clear();
        let mut changes = false;
        for dot in self.right.iter() {
            let next = (*dot / self.width * self.width) + (*dot % self.width + 1) % self.width;
            if self.grid[next] == '.' {
                self.right_next.push(next);
                changes = true;
            } else {
                self.right_next.push(*dot);
            }
            self.right_prev.push(*dot);
        }
        for i in 0..self.right_prev.len() {
            let tmp = self.grid[self.right_next[i]];
            self.grid[self.right_next[i]] = self.grid[self.right_prev[i]];
            self.grid[self.right_prev[i]] = tmp;
        }

        for dot in self.bottom.iter() {
            let next = (((*dot / self.width + 1) % self.height) * self.width) + (*dot % self.width);
            if self.grid[next] == '.' {
                self.bottom_next.push(next);
                changes = true;
            } else {
                self.bottom_next.push(*dot);
            }
            self.bottom_prev.push(*dot);
        }
        for i in 0..self.bottom_prev.len() {
            let tmp = self.grid[self.bottom_next[i]];
            self.grid[self.bottom_next[i]] = self.grid[self.bottom_prev[i]];
            self.grid[self.bottom_prev[i]] = tmp;
        }

        std::mem::swap(&mut self.right, &mut self.right_next);
        std::mem::swap(&mut self.bottom, &mut self.bottom_next);

        return changes;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y * self.width + x]);
            }
            println!("");
        }
        println!("");
    }
}

use aoc::utils;
fn main() {
    let input: Vec<String> = utils::read_list(&utils::get_input());
    let mut grid = Grid::new(&input);

    let mut cnt = 0;
    while grid.step() {
        cnt += 1;
    }
    println!("Part 1: {}", cnt + 1);
}
