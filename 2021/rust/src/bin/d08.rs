use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc::utils;

#[derive(Debug)]
struct Display {
    pattern: Vec<String>,
    output: Vec<String>,
}

fn guess_wiring(disp: &Display) -> HashMap<char, char> {
    let mut matches: [Vec<Box<HashSet<&str>>>; 10] = Default::default();
    let mut segments: [HashSet<&str>; 7] = Default::default();
    for n in disp.pattern.iter() {
        let segment: Box<HashSet<&str>> = Box::new(n.split_terminator("").skip(1).collect());
        match n.len() {
            2 => {
                matches[1].push(segment.clone());
            }
            3 => {
                matches[7].push(segment.clone());
            }
            4 => {
                matches[4].push(segment.clone());
            }
            5 => {
                matches[2].push(segment.clone());
                matches[3].push(segment.clone());
                matches[5].push(segment.clone());
            }
            6 => {
                matches[0].push(segment.clone());
                matches[6].push(segment.clone());
                matches[9].push(segment.clone());
            }
            7 => {
                matches[8].push(segment.clone());
            }
            _ => {}
        };
    }
    segments[1].extend(matches[1][0].iter());
    segments[2].extend(matches[1][0].iter());
    segments[0].extend(matches[7][0].difference(&matches[1][0]));
    segments[5].extend(matches[4][0].difference(&matches[1][0]));
    segments[6].extend(matches[4][0].difference(&matches[1][0]));

    //find 9
    let mut seg9 = matches[4][0].clone();
    seg9.extend(segments[0].iter());
    let _9: Vec<&Box<HashSet<&str>>> = matches[9]
        .iter()
        .filter(|s| s.intersection(&seg9).count() == seg9.len())
        .collect();

    segments[3].extend(_9[0].difference(&seg9));
    seg9.extend(_9[0].iter());
    segments[4].extend(matches[8][0].difference(&seg9));

    //find 2
    let mut seg2 = segments[0].clone();
    seg2.extend(segments[3].iter());
    seg2.extend(segments[4].iter());
    let _2: Vec<&Box<HashSet<&str>>> = matches[2]
        .iter()
        .filter(|s| s.intersection(&seg2).count() == seg2.len())
        .collect();

    segments[2] = &segments[2] - &_2[0];
    segments[1] = &segments[1] - &segments[2];
    segments[5] = &segments[5] - &_2[0];
    segments[6] = &segments[6] - &segments[5];

    let mut map = HashMap::new();
    let mut i = '0';
    for n in segments.iter() {
        let chr = (*n.iter().next().unwrap()).chars().next().unwrap();
        map.insert(chr, i);
        i = (i as u8 + 1) as char;
    }
    return map;
}

fn parse_display(line: &str) -> Display {
    let split: Vec<&str> = line.split(" | ").collect();
    let first: Vec<String> = split[0]
        .split_whitespace()
        .map(|x| String::from(x))
        .collect();
    let last: Vec<String> = split[1]
        .split_whitespace()
        .map(|x| String::from(x))
        .collect();
    return Display {
        pattern: first,
        output: last,
    };
}

fn parse_number(number: &str, map: &HashMap<char, char>, numbers: &HashMap<&str, i32>) -> i32 {
    let mut hash_c: Vec<char> = number.chars().map(|x| map[&x]).collect();
    hash_c.sort();
    let hash: String = hash_c.into_iter().collect();
    return *numbers.get(&hash.as_str()).unwrap();
}

fn parse_output(nbrs: &Vec<String>, map: &HashMap<char, char>) -> i32 {
    let mut numbers: HashMap<&str, i32> = HashMap::new();
    numbers.insert("012345", 0);
    numbers.insert("12", 1);
    numbers.insert("01346", 2);
    numbers.insert("01236", 3);
    numbers.insert("1256", 4);
    numbers.insert("02356", 5);
    numbers.insert("023456", 6);
    numbers.insert("012", 7);
    numbers.insert("0123456", 8);
    numbers.insert("012356", 9);
    let mut sum = 0;
    for n in nbrs.iter() {
        sum *= 10;
        sum += parse_number(&n, map, &numbers);
    }
    return sum;
}

fn main() {
    let list = utils::read_list_parse(&utils::get_input(), parse_display);
    let mut p1 = 0;
    let mut p2 = 0;
    for line in list.iter() {
        let wiring = guess_wiring(&line);
        p2 += parse_output(&line.output, &wiring);
        p1 += line.output.iter().fold(0, |tot, curr| {
            tot + match curr.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        });
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
