use aoc::helpers::coordinate::Coordinate;

type Coord = Coordinate<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    position: Coord,
    facing: Coord,
}

impl Guard {
    fn turn_right(&mut self) {
        self.facing = Coordinate::new(-self.facing.y, self.facing.x);
    }

    fn walk(&mut self) {
        self.position += self.facing;
    }

    fn next(&self) -> Coord {
        self.position + self.facing
    }
}

fn load(input: &str) -> (Guard, Vec<i32>, usize, usize) {
    let mut points = Vec::new();
    let mut guard = Guard {
        position: Coord::new(0, 0),
        facing: Coord::new(0, 0),
    };
    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().chars().count();
    points.resize(rows * columns, 0);
    for line in input.lines().enumerate() {
        let y = line.0;
        for c in line.1.chars().enumerate() {
            let x = c.0;
            match c.1 {
                '^' => {
                    guard = Guard {
                        position: Coord::new(x as i32, y as i32),
                        facing: Coord::new(0, -1),
                    };
                }
                '>' => {
                    guard = Guard {
                        position: Coord::new(x as i32, y as i32),
                        facing: Coord::new(1, 0),
                    };
                }
                'v' => {
                    guard = Guard {
                        position: Coord::new(x as i32, y as i32),
                        facing: Coord::new(0, 1),
                    };
                }
                '<' => {
                    guard = Guard {
                        position: Coord::new(x as i32, y as i32),
                        facing: Coord::new(-1, 0),
                    };
                }
                '#' => {
                    points[y * columns + x] = 1;
                }
                _ => {}
            }
        }
    }
    (guard, points, rows, columns)
}

fn solve(mut guard: Guard, points: &Vec<i32>, rows: usize, columns: usize) -> Option<Vec<i32>> {
    let mut visited = Vec::new();
    visited.resize(rows * columns, 0);

    while guard.position.x >= 0
        && guard.position.x < columns as i32
        && guard.position.y >= 0
        && guard.position.y < rows as i32
    {
        if visited[guard.position.y as usize * columns + guard.position.x as usize] > 3 {
            return None;
        }
        visited[guard.position.y as usize * columns + guard.position.x as usize] += 1;
        let next = guard.next();

        if next.x < 0 || next.x >= columns as i32 || next.y < 0 || next.y >= rows as i32 {
            break;
        }

        if points[next.y as usize * columns + next.x as usize] == 0 {
            guard.walk();
        } else {
            guard.turn_right();
        }
    }

    Some(visited)
}

fn part_one(input: &str) -> Option<usize> {
    let (guard, points, rows, columns) = load(input);

    let visited = solve(guard, &points, rows, columns).unwrap();
    Some(visited.iter().filter(|&x| *x > 0).count())
}

fn part_two(input: &str) -> Option<usize> {
    let (guard, mut points, rows, columns) = load(input);

    let visited = solve(guard, &points, rows, columns).unwrap();

    let mut total = 0;
    for cx in visited.iter().enumerate() {
        if *cx.1 == 0 {
            continue;
        }
        let x = cx.0 % columns;
        let y = cx.0 / columns;
        let c = Coord::new(x as i32, y as i32);
        if c == guard.position {
            continue;
        }
        points[c.y as usize * columns + c.x as usize] = 1;
        let newguard = guard.clone();
        let res = solve(newguard, &points, rows, columns);
        points[c.y as usize * columns + c.x as usize] = 0;
        if res.is_none() {
            total += 1;
        }
    }
    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 06);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 06);
        assert_eq!(part_one(&input), Some(41));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 06);
        assert_eq!(part_two(&input), Some(6));
    }
}
