use std::collections::HashSet;

use aoc2020::utils;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
struct Console {
    ip: i32,
    accumulator: i32,
    program: Vec<(Instruction, u8)>,
    flip: bool,
}

impl Console {
    fn new(code: &Vec<Instruction>) -> Console {
        let mut c = Console {
            ip: 0,
            accumulator: 0,
            program: Vec::new(),
            flip: false,
        };
        for inst in code.iter() {
            c.program.push((inst.clone(), 0));
        }
        return c;
    }

    fn run(&mut self) -> u32 {
        let pointer = self.program.get_mut(self.ip as usize);

        if pointer.is_none() {
            return 0;
        }

        let instr = pointer.unwrap();

        instr.1 += 1;

        let mut i = instr.0.clone();
        if self.flip {
            i = match i {
                Jmp(v) => Nop(v),
                Nop(v) => Jmp(v),
                _ => panic!(),
            };
            self.flip = false;
        }

        match i {
            Acc(val) => {
                self.accumulator += val;
                self.ip += 1;
            }
            Jmp(val) => {
                self.ip += val;
            }
            Nop(_v) => {
                self.ip += 1;
            }
        };
        return instr.1 as u32;
    }

    fn valid(&self) -> bool {
        return (self.ip >= 0) && (self.ip < self.program.len() as i32);
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.ip = 0;
        for i in self.program.iter_mut() {
            i.1 = 0;
        }
    }

    fn current(&self) -> Option<Instruction> {
        return match self.program.get(self.ip as usize) {
            Some(v) => Some(v.0),
            _ => None,
        };
    }

    fn clone_from(&mut self, source: &Console) {
        self.program.clone_from(&source.program);
        self.accumulator = source.accumulator;
        self.ip = source.ip;
    }
}

use Instruction::{Acc, Jmp, Nop};

fn parse_program(line: &str) -> Instruction {
    let instr = line.split(" ").collect::<Vec<&str>>();

    let opcode = instr.get(0).unwrap();
    let value = instr.get(1).unwrap().parse::<i32>().unwrap();

    return match *opcode {
        "acc" => Acc(value),
        "jmp" => Jmp(value),
        "nop" => Nop(value),
        _ => Nop(0),
    };
}

fn main() {
    let code = utils::read_list_parse("inputs/d08.txt", parse_program);
    let mut console = Console::new(&code);

    let mut acc = 0;
    while console.valid() && console.run() < 2 {
        acc = console.accumulator;
    }

    println!("Part 1: {}", acc);

    let mut ending = HashSet::new();
    let mut jmp = false;
    for i in (0..console.program.len()).rev() {
        let inst = console.program.get(i).unwrap().0;
        let end = match inst {
            Acc(_v) => jmp == false,
            Jmp(v) => {
                jmp = true;
                v + i as i32 == console.program.len() as i32
            }
            Nop(_v) => jmp == false,
        };
        if end {
            ending.insert(i);
        }
    }

    console.reset();
    let mut console2 = Console::new(&code);

    while console.valid() {
        if match console.current().unwrap() {
            Acc(_v) => false,
            _ => true,
        } {
            console2.clone_from(&console);
            console2.flip = true;
            while console2.valid() && console2.run() < 2 {
                acc = console2.accumulator;
            }

            if console2.ip == console2.program.len() as i32 {
                println!("Part 2: {}", acc);
                break;
            }
        }
        if console.run() > 1 {
            break;
        }
    }
}
