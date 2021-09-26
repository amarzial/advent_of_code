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
    cache: HashMap<usize, [Option<usize>; 8]>,
}

impl Grid {
    fn new(grid: &Vec<Vec<u8>>) -> Grid {
        let mut g = Grid {
            width: 0,
            height: 0,
            used: Grd::new(),
            empty: Grd::new(),
            count: Grd::new(),
            next: Grd::new(),
            changed: true,
            cache: HashMap::new(),
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

    fn find(&self, pos: usize, x_increment: i32, y_increment: i32) -> Option<usize> {
        let mut y = (pos / self.width) as i32;
        let mut x = (pos % self.width) as i32;
        loop {
            x += x_increment;
            y += y_increment;
            if x < 0 || x > (self.width - 1) as i32 || y < 0 || y > (self.height - 1) as i32 {
                return None;
            }
            let cursor = y * self.width as i32 + x;
            if *self.empty.get(&(cursor as usize)).unwrap_or(&0) == 0 {
                return Some(cursor as usize);
            }
        }
    }

    fn load(&mut self) {
        for pos in 0..(self.width * self.height) {
            if *self.empty.get(&pos).unwrap_or(&0) > 0 {
                continue;
            }
            let mut closest: [Option<usize>; 8] = [None; 8];
            closest[0] = self.find(pos, -1, -1);
            closest[1] = self.find(pos, 0, -1);
            closest[2] = self.find(pos, 1, -1);
            closest[3] = self.find(pos, -1, 0);
            closest[4] = self.find(pos, 1, 0);
            closest[5] = self.find(pos, -1, 1);
            closest[6] = self.find(pos, 0, 1);
            closest[7] = self.find(pos, 1, 1);
            self.cache.insert(pos, closest);
        }
    }

    fn fill2(&mut self) {
        self.count.clear();
        for k in self.used.keys() {
            if !self.cache.contains_key(k) {
                continue;
            }
            let closest = self.cache.get(k).unwrap();
            for item in closest.iter() {
                match item {
                    Some(val) => {
                        self.count
                            .insert(*val, self.count.get(val).unwrap_or(&0) + 1);
                    }
                    _ => {}
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

    fn run2(&mut self) {
        self.fill2();
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
            } else if cnt >= 5 {
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
    let input = utils::read_list_parse("inputs/d11.txt", |str| {
        return str.as_bytes().to_owned();
    });
    let mut grid = Grid::new(&input);

    loop {
        grid.run();
        if !grid.changed {
            break;
        }
    }
    println!("Part 1: {}", grid.count());

    grid = Grid::new(&input);
    grid.load();
    loop {
        grid.run2();
        if !grid.changed {
            break;
        }
    }
    println!("Part 2: {}", grid.count());
}
