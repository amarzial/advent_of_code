use std::collections::HashMap;

use itertools::Itertools;

type Int = i32;
struct Part {
    x: Int,
    m: Int,
    a: Int,
    s: Int,
}

struct Rule {
    target: Option<char>,
    operator: Option<char>,
    comparison: Option<Int>,
    destination: String,
}

fn parse_workflow(line: &str) -> Vec<Rule> {
    line.split(',')
        .map(|r| {
            if !r.contains(':') {
                Rule {
                    target: None,
                    operator: None,
                    comparison: None,
                    destination: r.to_owned(),
                }
            } else {
                let target = r.chars().nth(0).unwrap();
                let op = r.chars().nth(1).unwrap();
                let p = r.find(":").unwrap();
                let value = r[2..p].parse::<Int>().unwrap();
                let dest = &r[p + 1..];
                Rule {
                    target: Some(target),
                    operator: Some(op),
                    comparison: Some(value),
                    destination: dest.to_owned(),
                }
            }
        })
        .collect_vec()
}

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut lineit = input.lines();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    loop {
        let line = lineit.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let parts = line.split_once("{").unwrap();
        let workflow = parse_workflow(&parts.1[0..parts.1.len() - 1]);
        workflows.insert(parts.0.to_owned(), workflow);
    }

    let mut parts = vec![];
    for l in lineit {
        let p = l[1..l.len() - 1]
            .split(',')
            .map(|s| s.split('=').nth(1).unwrap().parse::<Int>().unwrap())
            .collect_vec();
        parts.push(Part {
            x: p[0],
            m: p[1],
            a: p[2],
            s: p[3],
        });
    }
    (workflows, parts)
}

fn process_workflow(part: &Part, workflows: &Vec<Rule>) -> String {
    for r in workflows {
        if r.operator.is_none() {
            return r.destination.to_owned();
        } else {
            let target = match r.target.unwrap() {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => unreachable!(),
            };
            match r.operator.unwrap() {
                '<' => {
                    if target < r.comparison.unwrap() {
                        return r.destination.to_owned();
                    }
                }
                '>' => {
                    if target > r.comparison.unwrap() {
                        return r.destination.to_owned();
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    unreachable!();
}

fn run(part: &Part, workflows: &HashMap<String, Vec<Rule>>) -> String {
    let mut rule = workflows.get("in").unwrap();
    loop {
        let res = process_workflow(part, rule);
        if res == "A" || res == "R" {
            return res.to_owned();
        }
        rule = workflows.get(&res).unwrap();
    }
}

fn part_one(input: &str) -> Option<Int> {
    let (workflows, parts) = parse_input(input);

    let mut sum = 0;
    for p in parts {
        let res = run(&p, &workflows);
        if res == "A" {
            sum += p.x + p.m + p.a + p.s;
        }
    }
    Some(sum)
}

fn part_two(_input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 19);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 19);
        assert_eq!(part_one(&input), Some(19114));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 19);
        assert_eq!(part_two(&input), None);
    }
}
