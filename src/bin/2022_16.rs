use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Valve {
    fr: i32,
    name: String,
    links: Vec<String>,
    distances: HashMap<String, usize>,
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    for l in input.split("\n").filter(|s| s.len() > 0) {
        let vals = aoc::utils::read_pattern("Valve {} has flow rate={}; {} to {} {}", l).unwrap();
        let links: Vec<String> = vals[4].split(", ").map(|v| String::from(v)).collect();
        valves.insert(
            String::from(vals[0]),
            Valve {
                fr: vals[1].parse().unwrap(),
                name: vals[0].to_string(),
                links,
                distances: HashMap::new(),
            },
        );
    }
    valves
}

fn calc_score(fr: i32, minutes_left: i32) -> i32 {
    fr * (minutes_left)
}

fn distances(node: String, map: &HashMap<String, Valve>) -> HashMap<String, usize> {
    map.keys()
        .filter(|s| *s != &node)
        .map(|target| {
            let dist = pathfinding::directed::bfs::bfs(
                &node,
                |s| map.get(s).unwrap().links.clone(),
                |x| x == target,
            )
            .unwrap();
            (target.to_string(), dist.len() - 1)
        })
        .collect()
}

fn find_path(
    current: String,
    valves: &HashMap<String, Valve>,
    next: Vec<String>,
    time: i32,
) -> i32 {
    let cur = valves.get(&current).unwrap();
    let score = calc_score(cur.fr, time);

    if time <= 0 {
        return 0;
    }

    let mut best = 0;
    for room in next.iter() {
        let others: Vec<String> = next.iter().cloned().filter(|v| v != room).collect();
        let res = find_path(
            room.to_string(),
            valves,
            others,
            time - *(cur.distances.get(room).unwrap()) as i32 - 1,
        );
        if res > best {
            best = res;
        }
    }
    score + best
}

fn part_one(input: &str) -> Option<i32> {
    let mut valves = parse_input(input);
    let map = valves.clone();
    let position = "AA";
    let timer = 30;

    for (n, v) in valves.iter_mut() {
        v.distances.extend(distances(n.to_string(), &map));
    }

    let possible_dest: Vec<String> = valves
        .values()
        .filter(|v| (v.name != "AA") && (v.fr > 0))
        .map(|v| v.name.to_string())
        .collect();

    let out = find_path(position.to_string(), &valves, possible_dest, timer);

    Some(out)
}

fn part_two(input: &str) -> Option<i32> {
    let mut valves = parse_input(input);
    let map = valves.clone();
    let position = "AA";
    let timer = 26;

    for (n, v) in valves.iter_mut() {
        v.distances.extend(distances(n.to_string(), &map));
    }

    let possible_dest: HashSet<String> = valves
        .values()
        .filter(|v| (v.name != "AA") && (v.fr > 0))
        .map(|v| v.name.to_string())
        .collect();

    let mut best = 0;
    for i in possible_dest
        .iter()
        .cloned()
        .combinations(possible_dest.len() / 2)
    {
        let first: HashSet<String> = i.iter().cloned().collect();
        let last: HashSet<String> = possible_dest.difference(&first).cloned().collect();
        let o1 = find_path(
            position.to_string(),
            &valves,
            first.into_iter().collect(),
            timer,
        );
        let o2 = find_path(
            position.to_string(),
            &valves,
            last.into_iter().collect(),
            timer,
        );
        best = best.max(o1 + o2);
    }

    Some(best)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 16);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
