use std::io::BufRead;

use regex::Regex;

type Line<'a> = (usize, usize, char, &'a str);

fn parse_line<'a>(line: &'a str, re: &Regex) -> Line<'a> {
    let caps = re.captures(line).expect(&line);
    let min = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = caps.get(3).unwrap().as_str().parse::<char>().unwrap();
    let pass = caps.get(4).unwrap().as_str();

    return (min, max, letter, pass);
}

fn parse_line_part1(line: Line) -> bool {
    let min = line.0;
    let max = line.1;
    let letter = line.2;
    let pass = line.3;

    let mut cnt = 0;
    for c in pass.chars() {
        if c == letter {
            cnt += 1;
        }
        if cnt > max {
            return false;
        }
    }
    if cnt >= min {
        return true;
    }
    return false;
}

fn parse_line_part2(line: Line) -> bool {
    let min = line.0;
    let max = line.1;
    let letter = line.2;
    let pass = line.3;

    let b = pass.as_bytes();

    return (b[min - 1] == letter as u8) != (b[max - 1] == letter as u8);
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or(String::from("inputs/d02.txt"));

    let f = std::fs::File::open(filename).unwrap();
    let mut buff = std::io::BufReader::new(f);
    let mut s = String::new();

    let reg = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut result1 = 0;
    let mut result2 = 0;
    while buff.read_line(&mut s).unwrap() > 0 {
        s.pop();
        let line = parse_line(&s, &reg);
        result1 += if parse_line_part1(line) { 1 } else { 0 };
        result2 += if parse_line_part2(line) { 1 } else { 0 };
        s.clear();
    }
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
