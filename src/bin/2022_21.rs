use std::{cell::RefCell, collections::HashMap};

#[derive(Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

type Int = i64;

#[derive(Clone)]
struct Monkey {
    val: Option<Int>,
    op: Option<Op>,
    lhs: String,
    rhs: String,
    humn: bool,
}

fn parse_monkey(input: &str) -> (String, RefCell<Monkey>) {
    let mut parts = input.split_whitespace();
    let mut n = parts.next().unwrap().chars();
    let v1 = parts.next();
    let op = parts.next();
    let v2 = parts.next();

    n.next_back();
    let name = n.as_str();

    let mon = match op {
        Some(c) => {
            let o = match c.chars().next().unwrap() {
                '+' => Op::Add,
                '-' => Op::Sub,
                '*' => Op::Mul,
                '/' => Op::Div,
                _ => {
                    panic!("invalid op")
                }
            };
            Monkey {
                val: None,
                op: Some(o),
                lhs: v1.unwrap().to_string(),
                rhs: v2.unwrap().to_string(),
                humn: false,
            }
        }
        None => Monkey {
            val: Some(v1.unwrap().parse().unwrap()),
            op: None,
            lhs: String::new(),
            rhs: String::new(),
            humn: name == "humn",
        },
    };
    (name.to_string(), RefCell::new(mon))
}

type MMap = HashMap<String, RefCell<Monkey>>;

fn calc(name: &str, monkeys: &MMap) -> Int {
    let val = monkeys.get(name).unwrap().borrow().val.clone();
    match val {
        Some(v) => v,
        None => {
            let mut mon = monkeys.get(name).unwrap().borrow_mut();
            let lhs = mon.lhs.clone();
            let rhs = mon.rhs.clone();
            let v = match mon.op.clone().unwrap() {
                Op::Add => calc(&lhs, monkeys) + calc(&rhs, monkeys),
                Op::Sub => calc(&lhs, monkeys) - calc(&rhs, monkeys),
                Op::Mul => calc(&lhs, monkeys) * calc(&rhs, monkeys),
                Op::Div => calc(&lhs, monkeys) / calc(&rhs, monkeys),
            };
            mon.val = Some(v);
            v
        }
    }
}

fn calc2(name: &str, monkeys: &MMap) -> (Int, bool) {
    let val = monkeys.get(name).unwrap().borrow().val.clone();
    match val {
        Some(v) => {
            let humn = name == "humn";
            if humn {
                monkeys.get(name).unwrap().borrow_mut().humn = humn;
            }
            (v, humn)
        }
        None => {
            let mut mon = monkeys.get(name).unwrap().borrow_mut();
            let lhs = calc2(&mon.lhs.clone(), monkeys);
            let rhs = calc2(&mon.rhs.clone(), monkeys);
            let v = match mon.op.clone().unwrap() {
                Op::Add => lhs.0 + rhs.0,
                Op::Sub => lhs.0 - rhs.0,
                Op::Mul => lhs.0 * rhs.0,
                Op::Div => lhs.0 / rhs.0,
            };
            mon.val = Some(v);
            mon.humn = lhs.1 || rhs.1;
            (v, mon.humn)
        }
    }
}

fn part_one(input: &str) -> Option<Int> {
    let list = aoc::utils::read_list_parse(input, parse_monkey);
    let monkeys: MMap = list.iter().cloned().collect();

    Some(calc("root", &monkeys))
}

fn backtrack(name: &str, value: f64, monkeys: &MMap) {
    let monkey = monkeys.get(name).unwrap().borrow().clone();
    if name == "humn" {
        monkeys.get(name).unwrap().borrow_mut().val = Some(value as Int);
        return;
    }

    let lhs = monkeys.get(&monkey.lhs).unwrap().borrow().clone();
    let rhs = monkeys.get(&monkey.rhs).unwrap().borrow().clone();

    match monkey.op.unwrap() {
        Op::Add => {
            if lhs.humn {
                backtrack(&monkey.lhs, value - rhs.val.unwrap() as f64, monkeys);
            } else {
                backtrack(&monkey.rhs, value - lhs.val.unwrap() as f64, monkeys);
            }
        }
        Op::Mul => {
            if lhs.humn {
                backtrack(&monkey.lhs, value / rhs.val.unwrap() as f64, monkeys);
            } else {
                backtrack(&monkey.rhs, value / lhs.val.unwrap() as f64, monkeys);
            }
        }
        Op::Sub => {
            if lhs.humn {
                backtrack(&monkey.lhs, value + rhs.val.unwrap() as f64, monkeys);
            } else {
                backtrack(&monkey.rhs, -value + lhs.val.unwrap() as f64, monkeys);
            }
        }
        Op::Div => {
            if lhs.humn {
                backtrack(&monkey.lhs, value * rhs.val.unwrap() as f64, monkeys);
            } else {
                backtrack(&monkey.rhs, lhs.val.unwrap() as f64 / value, monkeys);
            }
        }
    }
}

fn part_two(input: &str) -> Option<Int> {
    let list = aoc::utils::read_list_parse(input, parse_monkey);
    let monkeys: MMap = list.iter().cloned().collect();

    let root = monkeys.get("root").unwrap().borrow().clone();
    let lhs = calc2(&root.lhs, &monkeys);
    let rhs = calc2(&root.rhs, &monkeys);
    if lhs.1 {
        backtrack(&root.lhs, rhs.0 as f64, &monkeys);
    } else {
        backtrack(&root.rhs, lhs.0 as f64, &monkeys);
    }

    let v = monkeys.get("humn").unwrap().borrow().val.unwrap();
    Some(v)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 21);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
