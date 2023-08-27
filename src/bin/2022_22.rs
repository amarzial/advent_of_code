use aoc::helpers::coordinate::Coordinate;
use std::rc::Rc;

type Coord = Coordinate<Int>;

#[derive(Debug)]
struct Node {
    val: char,
    coord: Coord,
    neighbors: Vec<Rc<Node>>,
}

type Int = i32;

#[derive(Debug)]
enum Move {
    Forward(Int),
    Left,
    Right,
}

type NodeRef = Rc<Node>;

#[derive(Debug, Clone)]
struct Walker {
    position: Rc<Node>,
    facing: Coord,
}

fn connect_part_one(lines: &Vec<&str>) -> NodeRef {
    let mut root = None;

    let rows = lines.len();
    let cols = lines[0].len();

    let mut map: Vec<NodeRef> = vec![];

    for r in 0..rows {
        for c in 0..cols {
            let bt = lines[r].as_bytes().get(c).unwrap_or(&(' ' as u8));
            let n = Rc::new(Node {
                val: *bt as char,
                coord: Coord::new(c as Int, r as Int),
                neighbors: vec![],
            });
            if root.is_none() {
                root = Some(Rc::clone(&n));
            }

            map.push(Rc::clone(&n));
        }
    }

    println!("{:?}", map);
    root.unwrap()
}

fn parse_input(input: &str) -> (Node, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let map = parts.next().unwrap();
    let sequence = parts.next().unwrap();

    let lines = Vec::from_iter(map.lines());
    connect_part_one(&lines);

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
    let n = Node {
        val: 'K',
        coord: Coord::new(0, 0),
        neighbors: Vec::new(),
    };
    (n, moves)
}

fn part_one(input: &str) -> Option<i32> {
    let (map, moves) = parse_input(input);
    Some(3)
}

fn part_two(input: &str) -> Option<i32> {
    let (map, moves) = parse_input(input);
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
