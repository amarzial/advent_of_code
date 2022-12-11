use std::cell::RefCell;

type Int = u64;

#[derive(Debug)]
enum Operation {
    Add(Option<Int>),
    Mul(Option<Int>),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Int>,
    operation: Operation,
    test: Int,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl Monkey {
    fn load(txt: &str) -> Monkey {
        let parsed = aoc::utils::read_pattern(
            "Monkey {}:
  Starting items: {}
  Operation: new = old {} {}
  Test: divisible by {}
    If true: throw to monkey {}
    If false: throw to monkey {}",
            txt,
        )
        .unwrap();

        let src = parsed[3].parse();
        let item = if src.is_ok() {
            Some(src.unwrap())
        } else {
            None
        };
        Monkey {
            items: parsed[1]
                .split(", ")
                .map(|v| v.parse::<Int>().unwrap())
                .collect(),
            operation: match parsed[2] {
                "+" => Operation::Add(item),
                _ => Operation::Mul(item),
            },
            test: parsed[4].parse().unwrap(),
            if_true: parsed[5].parse().unwrap(),
            if_false: parsed[6].parse().unwrap(),
            inspections: 0,
        }
    }

    fn turn(&mut self, monkeys: &Vec<RefCell<Monkey>>, reduce: Int, modulo: Int) {
        for i in self.items.iter() {
            let mut v = match self.operation {
                Operation::Add(val) => i + val.unwrap_or(*i),
                Operation::Mul(val) => i * val.unwrap_or(*i),
            };
            if reduce > 0 {
                v /= reduce;
            }
            if modulo > 0 {
                v = v % modulo;
            }
            let target = if v % self.test == 0 {
                self.if_true
            } else {
                self.if_false
            };

            monkeys[target].borrow_mut().items.push(v);
            self.inspections += 1;
        }
        self.items.clear();
    }
}

fn part_one(input: &str) -> Option<usize> {
    let mut monkeys: Vec<RefCell<Monkey>> = input
        .split("\n\n")
        .map(|m| RefCell::new(Monkey::load(m.trim_end())))
        .collect();

    for _ in 0..20 {
        for n in 0..monkeys.len() {
            monkeys[n].borrow_mut().turn(&monkeys, 3, 0);
        }
    }

    monkeys.sort_by(|m1, m2| {
        m2.borrow()
            .inspections
            .partial_cmp(&m1.borrow().inspections)
            .unwrap()
    });

    let m1 = monkeys[0].borrow().inspections;
    let m2 = monkeys[1].borrow().inspections;
    Some(m1 * m2)
}

fn part_two(input: &str) -> Option<usize> {
    let mut monkeys: Vec<RefCell<Monkey>> = input
        .split("\n\n")
        .map(|m| RefCell::new(Monkey::load(m.trim_end())))
        .collect();

    let modulo = monkeys.iter().map(|m| m.borrow().test).product();

    for _ in 0..10000 {
        for n in 0..monkeys.len() {
            monkeys[n].borrow_mut().turn(&monkeys, 0, modulo);
        }
    }

    monkeys.sort_by(|m1, m2| {
        m2.borrow()
            .inspections
            .partial_cmp(&m1.borrow().inspections)
            .unwrap()
    });

    let m1 = monkeys[0].borrow().inspections;
    let m2 = monkeys[1].borrow().inspections;
    Some(m1 * m2)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 11);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
