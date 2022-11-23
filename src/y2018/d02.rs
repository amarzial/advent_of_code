use std::collections::HashMap;

use crate::day;
use crate::utils;

fn count(word: &str) -> (bool, bool) {
    let mut counter: HashMap<char, i32> = HashMap::new();
    for c in word.chars() {
        counter.insert(c, *counter.get(&c).unwrap_or(&0) + 1);
    }
    let mut two = false;
    let mut three = false;
    for v in counter.values() {
        if *v == 2 {
            two = true;
        }
        if *v == 3 {
            three = true;
        }
    }
    (two, three)
}

fn check(a: &str, b: &str) -> Option<String> {
    let ba = a.as_bytes();
    let bb = b.as_bytes();
    let mut diffs = 0;

    let mut pos = 0;
    for i in 0..ba.len() {
        if ba[i] != bb[i] {
            diffs += 1;
            pos = i;
            if diffs > 1 {
                return None;
            }
        }
    }
    let s = a[0..pos].to_string();
    let s2 = a[(pos + 1)..].to_string();
    return Some(s + &s2);
}

pub fn run(d: &mut day::Day) -> bool {
    let lines = utils::read_list::<String>(&d.input);
    let mut twos = 0;
    let mut threes = 0;
    for line in lines.iter() {
        let r = count(line);
        if r.0 {
            twos += 1;
        }
        if r.1 {
            threes += 1;
        }
    }

    d.set_part_1((twos * threes).to_string());

    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            match check(&lines[i], &lines[j]) {
                None => {}
                Some(s) => {
                    d.set_part_2(s);
                    return true;
                }
            }
        }
    }

    true
}
