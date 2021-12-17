use std::{collections::HashMap, hash::Hash};

use aoc::utils;

#[derive(Eq, Hash, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    let coords: Vec<&str> = line.split(" -> ").collect();
    let first: Vec<&str> = coords[0].split(',').collect();
    let p1 = Point {
        x: first[0].parse().unwrap(),
        y: first[1].parse().unwrap(),
    };
    let second: Vec<&str> = coords[1].split(',').collect();
    let p2 = Point {
        x: second[0].parse().unwrap(),
        y: second[1].parse().unwrap(),
    };

    return (p1, p2);
}

fn generate_grid(list: Vec<(Point, Point)>) -> HashMap<Point, i32> {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for line in list.iter() {
        if line.0.x == line.1.x {
            let from = std::cmp::min(line.0.y, line.1.y);
            let to = std::cmp::max(line.0.y, line.1.y);
            for i in from..=to {
                let p = Point { x: line.0.x, y: i };
                let v = map.get(&p).unwrap_or(&0) + 1;
                map.insert(p, v);
            }
        } else if line.0.y == line.1.y {
            let from = std::cmp::min(line.0.x, line.1.x);
            let to = std::cmp::max(line.0.x, line.1.x);
            for i in from..=to {
                let p = Point { x: i, y: line.0.y };
                let v = map.get(&p).unwrap_or(&0) + 1;
                map.insert(p, v);
            }
        }
    }
    return map;
}
fn generate_grid_diag(list: Vec<(Point, Point)>) -> HashMap<Point, i32> {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for line in list.iter() {
        let incr_x = (line.1.x - line.0.x).clamp(-1, 1);
        let incr_y = (line.1.y - line.0.y).clamp(-1, 1);

        let mut p = line.0.clone();
        loop {
            let pt = p.clone();
            let v = map.get(&pt).unwrap_or(&0) + 1;
            map.insert(pt, v);
            if p == line.1 {
                break;
            }
            p.x += incr_x;
            p.y += incr_y;
        }
    }
    return map;
}

fn main() {
    let grid = generate_grid(utils::read_list_parse(&utils::get_input(), parse_line));
    let grid2 = generate_grid_diag(utils::read_list_parse(&utils::get_input(), parse_line));

    println!(
        "Part 1: {}",
        grid.into_values()
            .fold(0, |sum, curr| { sum + if curr > 1 { 1 } else { 0 } })
    );
    println!(
        "Part 2: {}",
        grid2
            .into_values()
            .fold(0, |sum, curr| { sum + if curr > 1 { 1 } else { 0 } })
    );
}
