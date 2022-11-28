use std::collections::HashMap;

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

fn part_one(input: &str) -> Option<String> {
    let lines = aoc::utils::read_list::<String>(input);
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
    Some((twos * threes).to_string())
}

fn part_two(input: &str) -> Option<String> {
    let lines = aoc::utils::read_list::<String>(input);
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            match check(&lines[i], &lines[j]) {
                None => {}
                Some(s) => return Some(s),
            };
        }
    }
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2018, 02);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2018, 02);
        assert_eq!(part_one(&input), Some(12.to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2018, 02);
        assert_eq!(part_two(&input), Some(String::from("fgij")));
    }
}
