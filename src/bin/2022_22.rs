use aoc::helpers::coordinate::Coordinate;
use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

type Coord = Coordinate<Int>;

struct Node {
    val: char,
    coord: Coord,
    neighbors: [Option<Rc<RefCell<Node>>>; 4],
}

type Int = i32;

#[derive(Debug)]
enum Move {
    Forward(Int),
    Left,
    Right,
}

type NodeRef = Rc<RefCell<Node>>;

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node\n  val: {}\n  coord: {:?}\n  {}\n",
            self.val,
            self.coord,
            self.neighbors.iter().all(|x| x.is_some())
        )
    }
}

#[derive(Debug, Clone)]
struct Walker {
    position: NodeRef,
    facing: usize,
}

impl Walker {
    fn step(&mut self, action: &Move) {
        match action {
            Move::Forward(n) => {
                for _i in 0..*n {
                    let pos = Rc::clone(&self.position);
                    let tmp = pos.borrow().neighbors[self.facing].clone();
                    let r = tmp.unwrap();
                    let newpos = Rc::clone(&r);
                    if newpos.borrow().val == '.' {
                        self.position = newpos;
                    } else {
                        break;
                    }
                }
            }
            Move::Left => {
                if self.facing == 0 {
                    self.facing = 3;
                } else {
                    self.facing -= 1;
                }
            }
            Move::Right => {
                if self.facing == 3 {
                    self.facing = 0;
                } else {
                    self.facing += 1;
                }
            }
        }
    }
}

fn connect_part_one(lines: &Vec<&str>) -> NodeRef {
    let mut root = None;

    let rows = lines.len();
    let cols = lines[0].len();

    let mut map: Vec<NodeRef> = vec![];
    map.reserve(rows * cols);

    for r in 0..rows {
        for c in 0..cols {
            let bt = lines[r].as_bytes().get(c).unwrap_or(&(' ' as u8));
            let val = *bt as char;
            let n = Rc::new(RefCell::new(Node {
                val: val,
                coord: Coord::new(c as Int, r as Int),
                neighbors: [None, None, None, None],
            }));
            if root.is_none() && val == '.' {
                root = Some(Rc::clone(&n));
            }

            map.push(Rc::clone(&n));
        }
    }

    let neighbors = [
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(-1, 0),
        Coord::new(0, -1),
    ];

    for n in map.iter() {
        let mut i = 0;
        for offset in neighbors {
            let mut c = n.borrow().coord;
            loop {
                c += offset;
                if c.x >= (cols as Int) {
                    c.x = 0
                }
                if c.x < 0 {
                    c.x = (cols - 1) as Int;
                }
                if c.y >= (rows as Int) {
                    c.y = 0
                }
                if c.y < 0 {
                    c.y = (rows - 1) as Int;
                }
                let neighbor_node = Rc::clone(&map[(c.y * cols as Int + c.x) as usize]);
                if neighbor_node.borrow().val != ' ' {
                    n.borrow_mut().neighbors[i] = Some(Rc::clone(&neighbor_node));
                    break;
                }
            }
            i += 1;
        }
    }

    root.unwrap()
}

fn parse_input(input: &str) -> (NodeRef, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let map = parts.next().unwrap();
    let sequence = parts.next().unwrap().trim();

    let lines = Vec::from_iter(map.lines());
    let start = connect_part_one(&lines);

    let mut moves = vec![];
    let mut current = 0;
    let mut chrs = sequence.chars().peekable();
    while let Some(m) = chrs.next() {
        match m {
            '0'..='9' => {
                current *= 10;
                current += m as Int - '0' as Int;
                if let None = chrs.peek() {
                    moves.push(Move::Forward(current));
                }
            }
            dir => {
                if current > 0 {
                    moves.push(Move::Forward(current));
                    current = 0;
                };

                match dir {
                    'R' => {
                        moves.push(Move::Right);
                    }
                    _ => {
                        moves.push(Move::Left);
                    }
                };
            }
        }
    }
    (start, moves)
}

fn part_one(input: &str) -> Option<i32> {
    let (start, moves) = parse_input(input);

    let mut w = Walker {
        position: start,
        facing: 0,
    };

    for m in moves.iter() {
        w.step(m);
    }

    let final_pos = w.position.borrow().coord;

    Some((final_pos.y + 1) * 1000 + (final_pos.x + 1) * 4 + w.facing as Int)
}

fn part_two(input: &str) -> Option<i32> {
    // let (map, moves) = parse_input(input);
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 22);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        // let input = aoc::utils::load_input("examples", 2022, 22);
        // assert_eq!(part_two(&input), None);
    }
}
