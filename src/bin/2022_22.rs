use aoc::helpers::coordinate::Coordinate;

struct Grid {
    points: Vec<char>,
    width: usize,
    height: usize,
}

type Int = i32;

#[derive(Debug)]
enum Move {
    Forward(Int),
    Left,
    Right,
}

type Coord = Coordinate<Int>;

#[derive(Debug, Clone)]
struct Walker {
    position: Coord,
    facing: Coord,
}

fn parse_input(input: &str) -> (Grid, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let map = parts.next().unwrap();
    let sequence = parts.next().unwrap().trim();

    let lines = map.lines();
    let max_width = lines.clone().into_iter().map(|l| l.len()).max().unwrap();
    let height = lines.clone().into_iter().count();
    let mut grid = vec![];
    grid.resize(max_width * height, ' ');
    for line in lines.enumerate() {
        for c in line.1.chars().enumerate() {
            grid[line.0 * max_width + c.0] = c.1;
        }
    }

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
    (
        Grid {
            points: grid,
            width: max_width,
            height,
        },
        moves,
    )
}

fn step(mut next: Coord, facing: Coord, grid: &Grid) -> Coord {
    next += facing;
    next.x = next.x % grid.width as Int;
    while next.x < 0 {
        next.x += grid.width as Int;
    }
    next.y = next.y % grid.height as i32;
    while next.y < 0 {
        next.y += grid.height as Int;
    }
    next
}

fn next_position(walker: &mut Walker, grid: &Grid, steps: Int, history: &mut Vec<Walker>) {
    let mut next = walker.position;
    for _ in 0..steps {
        let prev = next;
        next = step(next, walker.facing, grid);
        while grid.points[(next.y * grid.width as Int + next.x) as usize] == ' ' {
            next = step(next, walker.facing, grid);
        }
        if grid.points[(next.y * grid.width as Int + next.x) as usize] == '#' {
            next = prev;
            walker.position = next;
            history.push(walker.clone());
            break;
        }
        walker.position = next;
        history.push(walker.clone());
    }
}

fn _print(grid: &Grid, history: &Vec<Walker>, walker: &Walker) {
    let mut path = vec![];
    path.resize(grid.points.len(), ' ');
    for h in history {
        let c = match (h.facing.x, h.facing.y) {
            (1, 0) => '>',
            (0, 1) => 'v',
            (-1, 0) => '<',
            (0, -1) => '^',
            _ => ' ',
        };
        path[h.position.y as usize * grid.width + h.position.x as usize] = c;
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            if x == walker.position.x as usize && y == walker.position.y as usize {
                print!("X");
            } else if path[y * grid.width as usize + x] != ' ' {
                print!("{}", path[y * grid.width as usize + x]);
            } else {
                print!("{}", grid.points[y * grid.width as usize + x]);
            }
        }
        println!("");
    }
}

fn part_one(input: &str) -> Option<i32> {
    let (map, moves) = parse_input(input);

    let start = map.points.iter().position(|p| *p == '.').unwrap();

    let mut w = Walker {
        position: Coord::new((start % map.width) as Int, (start / map.width) as Int),
        facing: Coordinate { x: 1, y: 0 },
    };

    let mut history = vec![];

    for m in moves {
        match m {
            Move::Left => w.facing = Coord::new(w.facing.y, w.facing.x) * Coord::new(1, -1),
            Move::Right => w.facing = Coord::new(w.facing.y, w.facing.x) * Coord::new(-1, 1),
            Move::Forward(steps) => {
                next_position(&mut w, &map, steps, &mut history);
            }
        };
    }

    let f = match (w.facing.x, w.facing.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => 0,
    };

    Some((w.position.y + 1) * 1000 + (w.position.x + 1) * 4 + f)
}

fn part_two(input: &str) -> Option<i32> {
    let (map, moves) = parse_input(input);

    let start = map.points.iter().position(|p| *p == '.').unwrap();

    let mut w = Walker {
        position: Coord::new((start % map.width) as Int, (start / map.width) as Int),
        facing: Coordinate { x: 1, y: 0 },
    };

    let mut history = vec![];

    for m in moves {
        match m {
            Move::Left => w.facing = Coord::new(w.facing.y, w.facing.x) * Coord::new(1, -1),
            Move::Right => w.facing = Coord::new(w.facing.y, w.facing.x) * Coord::new(-1, 1),
            Move::Forward(steps) => {
                next_position(&mut w, &map, steps, &mut history);
            }
        };
    }

    let f = match (w.facing.x, w.facing.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => 0,
    };

    Some((w.position.y + 1) * 1000 + (w.position.x + 1) * 4 + f)
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
        let input = aoc::utils::load_input("examples", 2022, 22);
        assert_eq!(part_two(&input), None);
    }
}
