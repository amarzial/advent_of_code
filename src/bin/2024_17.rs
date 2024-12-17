use aoc::utils::read_pattern;
use itertools::Itertools;
use pathfinding::prelude::dfs;

#[derive(Clone, Copy, Debug)]
enum Op {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc(u8),
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

#[derive(Debug, Clone)]
struct CPU {
    ip: usize,
    regs: [i64; 3],
    program: Vec<Op>,
    out_buf: String,
}

impl CPU {
    fn new(input: &str) -> CPU {
        let mut lines = input.lines();

        let reg_a = read_pattern("Register A: {}", lines.next().unwrap()).unwrap()[0]
            .parse()
            .unwrap();
        let reg_b = read_pattern("Register B: {}", lines.next().unwrap()).unwrap()[0]
            .parse()
            .unwrap();
        let reg_c = read_pattern("Register C: {}", lines.next().unwrap()).unwrap()[0]
            .parse()
            .unwrap();
        lines.next();
        let program = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect_vec();
        let mut opcodes = vec![];
        for i in (0..program.len()).step_by(2) {
            let op = match program[i] {
                0 => Op::Adv(program[i + 1]),
                1 => Op::Bxl(program[i + 1]),
                2 => Op::Bst(program[i + 1]),
                3 => Op::Jnz(program[i + 1]),
                4 => Op::Bxc(program[i + 1]),
                5 => Op::Out(program[i + 1]),
                6 => Op::Bdv(program[i + 1]),
                7 => Op::Cdv(program[i + 1]),
                _ => panic!("Invalid opcode"),
            };
            opcodes.push(op);
        }
        CPU {
            ip: 0,
            regs: [reg_a, reg_b, reg_c],
            program: opcodes,
            out_buf: String::new(),
        }
    }

    fn combo(&self, operand: u8) -> i64 {
        match operand {
            0 | 1 | 2 | 3 => operand as i64,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            7 => unreachable!(),
            _ => unreachable!(),
        }
    }

    fn cycle(&mut self) -> bool {
        if (self.ip as i32) < 0 || self.ip >= self.program.len() {
            return false;
        }
        let op = self.program[self.ip];
        let mut jumped = false;
        match op {
            Op::Adv(v) => {
                self.regs[0] = self.regs[0] / 2_f64.powf(self.combo(v) as f64) as i64;
            }
            Op::Bxl(v) => {
                self.regs[1] = self.regs[1] ^ v as i64;
            }
            Op::Bst(v) => {
                self.regs[1] = (self.combo(v) % 8) & 0b111;
            }
            Op::Jnz(v) => {
                if self.regs[0] != 0 {
                    self.ip = v as usize;
                    jumped = true;
                }
            }
            Op::Bxc(_v) => {
                self.regs[1] = self.regs[1] ^ self.regs[2];
            }
            Op::Out(v) => {
                self.out_buf += &format!("{}", self.combo(v) % 8);
            }
            Op::Bdv(v) => {
                self.regs[1] = self.regs[0] / 2_f64.powf(self.combo(v) as f64) as i64;
            }
            Op::Cdv(v) => {
                self.regs[2] = self.regs[0] / 2_f64.powf(self.combo(v) as f64) as i64;
            }
        }
        self.ip += if jumped { 0 } else { 1 };
        return true;
    }

    fn run(&mut self) {
        loop {
            if !self.cycle() {
                break;
            }
        }
    }
}

fn part_one(input: &str) -> Option<String> {
    let mut cpu = CPU::new(input);

    loop {
        if !cpu.cycle() {
            break;
        }
    }
    Some(cpu.out_buf.chars().join(","))
}

fn part_two(input: &str) -> Option<i64> {
    let cpu = CPU::new(input);
    let progstring = cpu
        .program
        .iter()
        .map(|op| match op {
            Op::Adv(v) => format!("0{}", v),
            Op::Bxl(v) => format!("1{}", v),
            Op::Bst(v) => format!("2{}", v),
            Op::Jnz(v) => format!("3{}", v),
            Op::Bxc(v) => format!("4{}", v),
            Op::Out(v) => format!("5{}", v),
            Op::Bdv(v) => format!("6{}", v),
            Op::Cdv(v) => format!("7{}", v),
        })
        .join("");

    let sol = dfs(
        0,
        |prev| {
            let n = *prev;
            (0..0o10).map(move |v| n * 0o10 + v).filter(|val| {
                let mut c = cpu.clone();
                c.regs[0] = *val;
                c.run();
                progstring.ends_with(&c.out_buf)
            })
        },
        |n| {
            let mut c = cpu.clone();
            c.regs[0] = *n;
            c.run();
            c.out_buf == progstring
        },
    );
    match sol {
        None => return None,
        Some(v) => Some(v[v.len() - 1]),
    }
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 17);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 17);
        assert_eq!(part_one(&input), Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 17);
        assert_eq!(part_two(&input), Some(117440));
    }
}
