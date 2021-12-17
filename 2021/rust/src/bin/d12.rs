use std::collections::{HashMap, HashSet};

use aoc::utils;

type Caves<'a> = HashMap<&'a str, Vec<&'a str>>;

fn trace(caves: &Caves, cave: &str, mut history: HashSet<String>) -> u32 {
    let mut count = 0;
    history.insert(cave.to_string());
    for link in caves[cave].iter() {
        if link.chars().next().unwrap().is_ascii_lowercase() && history.contains(&link.to_string())
        {
        } else if *link == "start" {
            count += 1;
        } else {
            count += trace(caves, *link, history.clone());
        }
    }
    return count;
}

fn trace2(caves: &Caves, cave: &str, mut history: HashSet<String>, twice: bool) -> u32 {
    let mut count = 0;
    history.insert(cave.to_string());
    for link in caves[cave].iter() {
        if link.chars().next().unwrap().is_ascii_lowercase() && history.contains(&link.to_string())
        {
            if !twice && (*link != "start") && (*link != "end") {
                count += trace2(caves, *link, history.clone(), true);
            }
        } else if *link == "start" {
            count += 1;
        } else {
            count += trace2(caves, *link, history.clone(), twice);
        }
    }
    return count;
}

fn main() {
    let input: Vec<String> = utils::read_list(&utils::get_input());

    let mut caves: Caves = HashMap::new();
    for l in input.iter() {
        let mut sides = l.split("-");
        let a = sides.next().unwrap();
        let b = sides.next().unwrap();
        {
            let e = caves.entry(a).or_insert(Vec::new());
            e.push(b);
        }
        {
            let e2 = caves.entry(b).or_insert(Vec::new());
            e2.push(a);
        }
    }

    let history: HashSet<String> = HashSet::new();
    let p1 = trace(&caves, "end", history.clone());
    let p2 = trace2(&caves, "end", history.clone(), false);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
