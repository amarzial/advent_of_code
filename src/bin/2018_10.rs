use std::{collections::HashSet, mem::swap};

#[derive(Debug, Clone, Copy)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_point(line: &str) -> Point {
    let pos_end = line.find('>').unwrap();
    let pos = &mut line[10..pos_end]
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap());

    let px = pos.next().unwrap();
    let py = pos.next().unwrap();

    let vel_end = line.rfind('<').unwrap();
    let vel = &mut line[vel_end + 1..line.len() - 1]
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap());

    let vx = vel.next().unwrap();
    let vy = vel.next().unwrap();

    Point {
        position: (px, py),
        velocity: (vx, vy),
    }
}

fn cycle(points: &mut Vec<Point>) {
    for mut p in points {
        p.position.0 += p.velocity.0;
        p.position.1 += p.velocity.1;
    }
}

fn height(points: &Vec<Point>) -> u32 {
    let mut min = 0;
    let mut max = 0;
    for p in points {
        min = p.position.1.min(min);
        max = p.position.1.max(max);
    }
    max.abs_diff(min)
}

fn print_grid(points: &Vec<Point>) -> String {
    let mut pts = HashSet::new();
    let mut tl = (i32::MAX, i32::MAX);
    let mut br = (i32::MIN, i32::MIN);
    for p in points {
        pts.insert(p.position);
        tl.0 = tl.0.min(p.position.0);
        tl.1 = tl.1.min(p.position.1);
        br.0 = br.0.max(p.position.0);
        br.1 = br.1.max(p.position.1);
    }

    let mut out = String::from("\n");

    for y in tl.1..=br.1 {
        for x in tl.0..=br.0 {
            out += &format!(
                "{}",
                match pts.contains(&(x, y)) {
                    true => "#",
                    false => ".",
                }
            )
        }
        out += "\n";
    }
    out
}

fn part_one(input: &str) -> Option<String> {
    let mut points = aoc::utils::read_list_parse(input, parse_point);
    let mut points2: Vec<Point> = Vec::clone(&points);

    let mut h = height(&points);
    let mut previous = h;

    let mut seconds = -1;
    while h <= previous {
        points2 = points.clone();
        cycle(&mut points);
        previous = h;
        h = height(&points);
        seconds += 1;
    }
    aoc::utils::cache(Some(seconds.to_string()));
    Some(print_grid(&points2))
}

fn part_two(input: &str) -> Option<String> {
    match aoc::utils::cache(None) {
        Some(s) => Some(s),
        None => {
            part_one(input);
            aoc::utils::cache(None)
        }
    }
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2018, 10);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2018, 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2018, 10);
        assert_eq!(part_two(&input), Some(3.to_string()));
    }
}
