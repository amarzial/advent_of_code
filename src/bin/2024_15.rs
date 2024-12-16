use std::collections::{HashMap, HashSet};

use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;
type Coord = Coordinate<i32>;

fn parse_input(input: &str) -> (Coord, HashMap<Coord, char>, Vec<char>) {
    let input_split = input.split("\n\n").collect_vec();
    let mut map = HashMap::new();
    let mut bot = Coord::new(0, 0);
    for (y, line) in input_split[0].lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord::new(x as i32, y as i32);
            if c == '#' || c == 'O' {
                map.insert(coord, c);
            } else if c == '@' {
                bot = coord;
            }
        }
    }
    let directions = input_split[1]
        .lines()
        .map(|f| f.chars())
        .flatten()
        .collect_vec();
    (bot, map, directions)
}

fn step(map: &mut HashMap<Coord, char>, bot: Coord, direction: char) -> Coord {
    let dir = match direction {
        '^' => Coord::new(0, -1),
        'v' => Coord::new(0, 1),
        '>' => Coord::new(1, 0),
        '<' => Coord::new(-1, 0),
        _ => Coord::new(0, 0),
    };
    let mut cursor = bot;
    loop {
        cursor += dir;
        let item = map.get(&cursor);
        if let Some('O') = item {
            continue;
        } else if let Some('#') = item {
            return bot;
        } else {
            break;
        }
    }

    if cursor == bot {
        return bot;
    }
    let newbot = bot + dir;
    loop {
        if cursor == newbot {
            break;
        }
        map.insert(cursor, *map.get(&(cursor - dir)).unwrap());
        map.remove(&(cursor - dir));
        cursor -= dir;
    }

    cursor
}

fn map_move(map: &mut HashMap<Coord, char>, dest: Coord, src: Coord) {
    let item = *map.get(&src).unwrap();

    let mut tmp = vec![item];
    if item == '[' {
        tmp.push(map.get(&(src + Coord::new(1, 0))).unwrap().clone());
    }

    map.remove(&src);
    if item == '[' {
        map.remove(&(src + Coord::new(1, 0)));
        map.insert(dest + Coord::new(1, 0), tmp[1]);
    }
    map.insert(dest, tmp[0]);
}

fn _print(width: usize, height: usize, map: &HashMap<Coord, char>, bot: Coord) {
    for y in 0..height {
        for x in 0..width {
            if bot.x == x as i32 && bot.y == y as i32 {
                print!("@");
            } else {
                print!(
                    "{}",
                    map.get(&Coord::new(x as i32, y as i32)).unwrap_or(&'.')
                );
            }
        }
        println!();
    }
    println!();
}

fn part_one(input: &str) -> Option<i32> {
    let (mut bot, mut map, directions) = parse_input(input);
    for direction in directions {
        bot = step(&mut map, bot, direction);
    }

    let mut tot = 0;
    for p in map {
        if p.1 == 'O' {
            tot += p.0.y * 100 + p.0.x;
        }
    }

    Some(tot)
}

fn build_map2(map: &HashMap<Coord, char>) -> HashMap<Coord, char> {
    let mut new_map = HashMap::new();
    for (coord, c) in map {
        let new_coord = *coord * Coordinate::new(2, 1);
        match c {
            '#' => {
                new_map.insert(new_coord, '#');
                new_map.insert(new_coord + Coord::new(1, 0), '#');
            }
            'O' => {
                new_map.insert(new_coord, '[');
                new_map.insert(new_coord + Coord::new(1, 0), ']');
            }
            _ => {}
        };
    }
    new_map
}

fn dir(c: char) -> Coord {
    match c {
        '^' => Coord::new(0, -1),
        'v' => Coord::new(0, 1),
        '>' => Coord::new(1, 0),
        '<' => Coord::new(-1, 0),
        _ => Coord::new(0, 0),
    }
}

fn pushed(map: &HashMap<Coord, char>, mut item: Coord, dir: Coord) -> Vec<Coord> {
    let mut moved = vec![];
    let current = *map.get(&item).unwrap_or(&'@');
    let size = if current == '@' {
        1
    } else {
        if dir.x == 0 {
            2
        } else {
            1
        }
    };
    if current == ']' {
        item -= Coord::new(1, 0);
    }

    let next = item
        + dir
        + if current == '[' && dir.x == 1 {
            Coord::new(1, 0)
        } else {
            Coord::new(0, 0)
        };
    let target = map.get(&next);
    match target {
        Some('[') | Some(']') => {
            moved.push(item);
            let chain = pushed(map, next, dir);
            if chain.is_empty() {
                return vec![];
            }
            moved.extend(chain);
        }
        Some('#') => {
            return vec![];
        }
        None => {
            moved.push(item);
        }
        _ => {}
    }
    if size == 2 {
        let next2 = next + Coord::new(1, 0);
        let target = map.get(&next2);
        match target {
            Some('[') => {
                let chain = pushed(map, next2, dir);
                if chain.is_empty() {
                    return vec![];
                }
                moved.extend(chain);
            }
            Some(']') => {}
            Some('#') => {
                return vec![];
            }
            None => {}
            _ => {}
        }
    }
    moved
}

fn part_two(input: &str) -> Option<i32> {
    let (mut bot, mut map, directions) = parse_input(input);
    map = build_map2(&map);
    bot *= Coordinate::new(2, 1);
    for (_cnt, direction) in directions.iter().enumerate() {
        let d = dir(*direction);
        let pushes = pushed(&map, bot, d);
        if pushes.len() > 0 {
            bot = pushes[0] + d;
            let mut others = HashSet::new();
            for push in pushes.iter().skip(1).rev() {
                if others.contains(push) {
                    continue;
                }
                others.insert(*push);
                map_move(&mut map, *push + d, *push);
            }
        }
    }

    let mut tot = 0;
    for p in map {
        if p.1 == '[' {
            tot += p.0.y * 100 + p.0.x;
        }
    }

    Some(tot)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 15);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 15);
        assert_eq!(part_one(&input), Some(10092));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 15);
        assert_eq!(part_two(&input), Some(9021));
    }
}
