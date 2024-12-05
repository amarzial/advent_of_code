use std::collections::HashMap;

type Rules = HashMap<i32, Vec<i32>>;
type Sequence = Vec<i32>;

fn parse_input(input: &str) -> (Rules, Vec<Sequence>) {
    let mut rules: Rules = HashMap::new();
    let mut sequences = Vec::new();

    let mut reading_rules = true;
    for line in input.lines() {
        if line.len() == 0 {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let mut parts = line.split("|");
            let p1 = parts.next().unwrap().trim().parse().unwrap();
            let p2 = parts.next().unwrap().trim().parse().unwrap();
            if rules.contains_key(&p1) {
                rules.get_mut(&p1).unwrap().push(p2);
            } else {
                rules.insert(p1, vec![p2]);
            }
        } else {
            sequences.push(line.split(",").map(|n| n.parse().unwrap()).collect());
        }
    }
    (rules, sequences)
}

fn is_ordered(seq: &Sequence, rules: &Rules) -> bool {
    let mut prev = seq[0];
    for n in seq[1..].iter() {
        if rules.contains_key(n) {
            for r in rules.get(n).unwrap() {
                if prev == *r {
                    return false;
                }
            }
        }
        prev = *n;
    }
    true
}

fn part_one(input: &str) -> Option<i32> {
    let (rules, sequences) = parse_input(input);
    let mut total = 0;

    for seq in sequences {
        if is_ordered(&seq, &rules) {
            let middle = seq.len() / 2;
            total += seq[middle];
        }
    }
    Some(total)
}

fn part_two(input: &str) -> Option<i32> {
    let (rules, sequences) = parse_input(input);
    let mut total = 0;

    for seq in sequences {
        if !is_ordered(&seq, &rules) {
            let mut cloned = seq.clone();
            cloned.sort_by(|a, b| {
                if rules.contains_key(a) {
                    let a_rules = rules.get(a).unwrap();
                    for r in a_rules {
                        if *r == *b {
                            return std::cmp::Ordering::Less;
                        }
                    }
                }
                if rules.contains_key(b) {
                    let b_rules = rules.get(b).unwrap();
                    for r in b_rules {
                        if *r == *a {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                }
                std::cmp::Ordering::Equal
            });
            let middle = cloned.len() / 2;
            total += cloned[middle];
        }
    }
    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 05);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 05);
        assert_eq!(part_one(&input), Some(143));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 05);
        assert_eq!(part_two(&input), Some(123));
    }
}
