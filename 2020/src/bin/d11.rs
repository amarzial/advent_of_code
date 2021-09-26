use aoc2020::utils;
use std::collections::HashMap;
use std::fmt;

type Grd = HashMap<usize, u8>;
struct Grid {
    width: usize,
    height: usize,
    used: Grd,
    empty: Grd,
    count: Grd,
    next: Grd,
    changed: bool,
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Grid {
        let mut g = Grid {
            width: 0,
            height: 0,
            used: Grd::new(),
            empty: Grd::new(),
            count: Grd::new(),
            next: Grd::new(),
            changed: true,
        };

        g.height = grid.len();
        g.width = grid.get(0).unwrap().len();
        for i in 0..grid.len() {
            let row = grid.get(i).unwrap();
            for j in 0..row.len() {
                let c = row.get(j).unwrap();
                if *c == b'L' {
                    // g.used.insert(i * row.len() + j, 1);
                } else {
                    g.empty.insert(i * row.len() + j, 1);
                }
            }
        }
        return g;
    }

    fn fill(&mut self) {
        self.count.clear();
        for k in self.used.keys() {
            let y = k / self.width;
            let x = k % self.width;
            let mut position;
            if y > 0 {
                //top
                position = (y - 1) * self.width + x;
                self.count
                    .insert(position, self.count.get(&position).unwrap_or(&0) + 1);

                if x > 0 {
                    //top left
                    position = (y - 1) * self.width + (x - 1);
                    self.count
                        .insert(position, self.count.get(&position).unwrap_or(&0) + 1);
                }
                if x < self.width - 1 {
                    //top right
                    position = (y - 1) * self.width + (x + 1);
                    self.count
                        .insert(position, self.count.get(&position).unwrap_or(&0) + 1);
                }
            }
            if x > 0 {
                //left
                position = y * self.width + (x - 1);
                self.count
                    .insert(position, self.count.get(&position).unwrap_or(&0) + 1);
            }
            if x < self.width - 1 {
                //right
                position = y * self.width + (x + 1);
                self.count
                    .insert(position, self.count.get(&position).unwrap_or(&0) + 1);
            }
            if y < self.height - 1 {
                //bottom
                position = (y + 1) * self.width + x;
                self.count
                    .insert(position, self.count.get(&position).unwrap_or(&0) + 1);

                if x > 0 {
                    //bottom left
                    position = (y + 1) * self.width + (x - 1);
                    self.count
                        .insert(position, self.count.get(&position).unwrap_or(&0) + 1);
                }
                if x < self.width - 1 {
                    //bottom right
                    position = (y + 1) * self.width + (x + 1);
                    self.count
                        .insert(position, self.count.get(&position).unwrap_or(&0) + 1);
                }
            }
        }
    }

    fn run(&mut self) {
        self.fill();
        self.next.clear();
        self.changed = false;
        for pos in 0..(self.width * self.height) {
            if *self.empty.get(&pos).unwrap_or(&0) > 0 {
                continue;
            }
            let current = *self.used.get(&pos).unwrap_or(&0);
            let cnt = *self.count.get(&pos).unwrap_or(&0);
            if cnt == 0 {
                self.next.insert(pos, 1);
                self.changed |= current == 0;
            } else if cnt >= 4 {
                self.next.remove(&pos);
                self.changed |= current > 0;
            } else if self.used.contains_key(&pos) {
                self.next.insert(pos, 1);
            }
        }
        self.used.clone_from(&self.next);
    }

    fn count(&self) -> usize {
        let mut cnt = 0;
        for pos in 0..(self.width * self.height) {
            if (*self.used.get(&pos).unwrap_or(&0) > 0)
                && (*self.empty.get(&pos).unwrap_or(&0) == 0)
            {
                cnt += 1;
            }
        }
        cnt
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let pos = i * self.width + j;
                if *self.empty.get(&pos).unwrap_or(&0) > 0 {
                    write!(f, ".").unwrap();
                } else {
                    let cnt = *self.count.get(&pos).unwrap_or(&0);
                    write!(f, "{}", cnt).unwrap();
                }
            }
            write!(f, "    ").unwrap();

            for j in 0..self.width {
                let pos = i * self.width + j;
                if *self.empty.get(&pos).unwrap_or(&0) > 0 {
                    write!(f, ".").unwrap();
                } else {
                    let cnt = *self.used.get(&pos).unwrap_or(&0);
                    write!(f, "{}", if cnt > 0 { '#' } else { 'L' }).unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n").unwrap();
        return Ok(());
    }
}

fn main() {
    let mut grid = Grid::new(utils::read_list_parse("inputs/d11.txt", |str| {
        return str.as_bytes().to_owned();
    }));
    loop {
        grid.run();
        if !grid.changed {
            break;
        }
    }
    println!("Part 1: {}", grid.count());
}
