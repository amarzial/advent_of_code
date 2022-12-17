use std::collections::HashSet;

use aoc::helpers::Coordinate;

type Int = i32;

type Coord = Coordinate<Int>;

#[derive(Clone)]
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
    grid: HashSet<Coord>,
    pieces: Vec<Piece>,

    current_pat: usize,
    current_piece: usize,
    top_rock: usize,
}

impl Tower {
    fn new(pat: String, pcs: Vec<Piece>) -> Tower {
        Tower {
            pattern: pat,
            grid: HashSet::new(),
            pieces: pcs,
            current_pat: 0,
            current_piece: 0,
            top_rock: 0,
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
        let p = self.pattern.as_bytes()[self.current_piece];
        self.current_pat += 1;
        if self.current_pat >= self.pattern.len() {
            self.current_pat = 0;
        }
        p
    }

    fn print(&self, pos: Coord, pc: &Piece) {
        for i in (0..20).rev() {
            for x in (0..7) {
                let pt = Coord::new(x, i);
                let mut c = if self.grid.contains(&pt) { '#' } else { '.' };

                if pc.data.iter().any(|p| pos + *p == pt) {
                    c = '#';
                }

                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }

    fn step(&mut self, mut pos: Coord, piece: &Piece) -> Option<Coord> {
        let mov = self.next_pattern();

        match mov {
            b'>' => {
                pos.x = i32::min(7 - piece.width as i32, pos.x + 1);
            }
            _ => {
                pos.x = i32::max(0, pos.x - 1);
            }
        };

        pos.y -= 1;

        self.print(pos, piece);

        if pos.y - (piece.height as Int - 1) == 0 {
            return None;
        }
        Some(pos)
        // for c in piece.data.iter() {}
    }

    fn run(&mut self) {
        let p = self.next_piece();

        let mut pos = Coord::new(2, (self.top_rock + 3 + p.height - 1) as i32);

        loop {
            let r = self.step(pos, &p);
            match r {
                Some(p) => pos = p,
                None => {
                    break;
                }
            }
        }
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

fn part_one(input: &str) -> Option<String> {
    let mut tow = Tower::new(input.trim().to_string(), load_pieces());
    tow.run();

    None
}

fn part_two(input: &str) -> Option<String> {
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 17);
        assert_eq!(part_two(&input), None);
    }
}
