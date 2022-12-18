use std::collections::HashMap;

use aoc::helpers::coordinate::Coordinate;

type Int = i32;

type Coord = Coordinate<Int>;

#[derive(Debug, Clone)]
struct Piece {
    data: Vec<Coord>,
    width: usize,
    height: usize,
}

fn parse_piece(str: &str) -> Piece {
    let mut v = vec![];
    let mut w = 0;
    let mut h = 0;
    for l in str.lines().enumerate() {
        for c in l.1.chars().enumerate() {
            match c.1 {
                '#' => {
                    w = w.max(c.0 + 1);
                    h = h.max(l.0 + 1);
                    v.push(Coord::new(c.0 as Int, l.0 as Int));
                }
                _ => {}
            }
        }
    }
    Piece {
        data: v,
        width: w,
        height: h,
    }
}

struct Tower {
    pattern: String,
    grid: Vec<u8>,
    pieces: Vec<Piece>,

    current_pat: usize,
    current_piece: usize,
    top_rock: Int,
    block_count: usize,
}

impl Tower {
    fn new(pat: String, pcs: Vec<Piece>) -> Tower {
        let mut grid: Vec<u8> = Vec::new();
        grid.resize(10000, 0);

        Tower {
            pattern: pat,
            grid,
            pieces: pcs,
            current_pat: 0,
            current_piece: 0,
            top_rock: 0,
            block_count: 0,
        }
    }

    fn next_piece(&mut self) -> Piece {
        let p = self.pieces[self.current_piece].clone();
        self.current_piece += 1;
        if self.current_piece >= self.pieces.len() {
            self.current_piece = 0;
        }
        p
    }

    fn next_pattern(&mut self) -> u8 {
        let p = self.pattern.as_bytes()[self.current_pat];
        self.current_pat += 1;
        if self.current_pat >= self.pattern.len() {
            self.current_pat = 0;
        }
        p
    }

    fn _print(&self) {
        for i in (0..30).rev() {
            let row = match self.grid.get(i) {
                Some(v) => (0..7)
                    .map(|x| if (v & (1 << x)) != 0 { '#' } else { '.' })
                    .collect::<String>(),
                None => String::from(".".repeat(7)),
            };
            println!("{}", row);
        }
        println!("");
    }

    fn store(&mut self, pos: Coord, piece: &Piece) {
        let inv = Coord::new(1, -1);
        for c in piece.data.iter() {
            let p = pos + *c * inv;
            let x = (1 as u8) << p.x;
            self.grid[p.y as usize] |= x;
        }
        self.top_rock = self.top_rock.max(pos.y + 1);
    }

    fn check_grid(&self, pos: Coord) -> bool {
        let x = (1 as u8) << pos.x;
        self.grid[pos.y as usize] & x != 0
    }

    fn check_walls(&self, pos: Coord, piece: &Piece) -> bool {
        let inv = Coord::new(1, -1);
        if (pos.x < 0) || ((7 - piece.width as Int) < pos.x) {
            return true;
        }
        piece
            .data
            .iter()
            .map(|p| pos + (*p * inv))
            .any(|p| self.check_grid(p))
    }

    fn check_collision(&self, mut pos: Coord, piece: &Piece) -> bool {
        let inv = Coord::new(1, -1);
        pos.y -= 1;

        if pos.y - (piece.height as Int - 1) < 0 {
            return true;
        }
        piece
            .data
            .iter()
            .map(|p| pos + (*p * inv))
            .any(|p| self.check_grid(p))
    }

    fn step(&mut self, mut pos: Coord, piece: &Piece) -> (Coord, bool) {
        let mov = self.next_pattern();

        let movement = if mov == b'>' {
            Coord::new(1, 0)
        } else {
            Coord::new(-1, 0)
        };

        pos += movement;
        if self.check_walls(pos, piece) {
            pos -= movement;
        }

        if self.check_collision(pos, piece) {
            self.store(pos, piece);
            return (pos, false);
        }
        pos.y -= 1;
        (pos, true)
    }

    fn run(&mut self) {
        let p = self.next_piece();

        let mut pos = Coord::new(2, (self.top_rock as usize + 3 + p.height - 1) as i32);

        loop {
            let r = self.step(pos, &p);
            pos = r.0;
            if !r.1 {
                break;
            }
        }
        self.block_count += 1;
    }

    fn hash(&self) -> Option<(usize, usize, Vec<u8>)> {
        if self.top_rock < 1500 {
            return None;
        };
        Some((
            self.current_pat,
            self.current_piece,
            self.grid
                .get((self.top_rock as usize - 4)..(self.top_rock as usize))
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<u8>>(),
        ))
    }
}

fn load_pieces() -> Vec<Piece> {
    vec![
        parse_piece("####"),
        parse_piece(".#.\n###\n.#."),
        parse_piece("..#\n..#\n###"),
        parse_piece("#\n#\n#\n#"),
        parse_piece("##\n##"),
    ]
}

fn part_one(input: &str) -> Option<Int> {
    let mut tow = Tower::new(input.trim().to_string(), load_pieces());
    for _ in 0..2022 {
        tow.run();
    }
    Some(tow.top_rock)
}

fn loop_detect(tow: &mut Tower) -> (usize, usize) {
    let mut set = HashMap::new();

    for i in 0..100000 {
        tow.run();
        if let Some(h) = tow.hash() {
            if !set.contains_key(&h) {
                set.insert(h, (i, tow.top_rock));
            } else {
                let v = set.get(&h).unwrap();
                return (i - v.0, (tow.top_rock - v.1) as usize);
            }
        }
    }
    (0, 0)
}

fn part_two(input: &str) -> Option<usize> {
    let mut tow = Tower::new(input.trim().to_string(), load_pieces());

    let (period, height) = loop_detect(&mut tow);

    let total_blocks: usize = 1000000000000;
    let cycles = (total_blocks - tow.block_count) / period;
    let additional_heigth = height * cycles;
    let remaining = total_blocks - (cycles * period) - tow.block_count;

    for _ in 0..remaining {
        tow.run();
    }

    Some(tow.top_rock as usize + additional_heigth)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 17);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
