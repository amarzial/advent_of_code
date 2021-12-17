use aoc::utils;
enum Direction {
    Down(i32),
    Up(i32),
    Forward(i32),
}

fn parse_direction(line: &str) -> Direction {
    let words: Vec<&str> = line.split(' ').collect();
    match words[0] {
        "down" => {
            return Direction::Down(str::parse(words[1]).unwrap());
        }
        "up" => {
            return Direction::Up(str::parse(words[1]).unwrap());
        }
        "forward" => {
            return Direction::Forward(str::parse(words[1]).unwrap());
        }
        _ => panic!(),
    }
}

fn main() {
    let filename = utils::get_input();
    let directions: Vec<Direction> = utils::read_list_parse(&filename, parse_direction);

    let mut x = 0;
    let mut y = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut aim2 = 0;
    for dir in directions.iter() {
        match *dir {
            Direction::Down(n) => {
                y += n;
                aim2 += n;
            }
            Direction::Up(n) => {
                y -= n;
                aim2 -= n;
            }
            Direction::Forward(n) => {
                x += n;
                x2 += n;
                y2 += n * aim2;
            }
        }
    }
    println!("Part 1: {}", x * y);
    println!("Part 2: {}", x2 * y2);
}
