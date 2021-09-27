#[derive(Copy, Clone, Debug)]
enum Instruction {
    N(i32),
    S(i32),
    W(i32),
    E(i32),
    L(i32),
    R(i32),
    F(i32),
}

struct Ferry {
    facing: i32,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Ferry2 {
    waypoint_x: i32,
    waypoint_y: i32,
    x: i32,
    y: i32,
}

use std::fmt::Debug;

use Instruction::{E, F, L, N, R, S, W};

impl Ferry {
    fn rotate(&mut self, inst: Instruction) {
        match inst {
            L(val) => self.facing -= val,
            R(val) => self.facing += val,
            _ => panic!("caso non previsto"),
        }
        self.facing %= 360;
        if self.facing < 0 {
            self.facing += 360;
        }
    }

    fn forward(&mut self, amount: i32) {
        match self.facing {
            0 => self.exec(N(amount)),
            90 => self.exec(E(amount)),
            180 => self.exec(S(amount)),
            270 => self.exec(W(amount)),
            _ => panic!("caso non previsto"),
        }
    }

    fn exec(&mut self, inst: Instruction) {
        match inst {
            N(val) => {
                self.y += val;
            }
            S(val) => {
                self.y -= val;
            }
            W(val) => {
                self.x -= val;
            }
            E(val) => {
                self.x += val;
            }
            L(_val) | R(_val) => self.rotate(inst),
            F(val) => self.forward(val),
        }
    }
}

impl Ferry2 {
    fn rotate(&mut self, inst: Instruction) {
        let rotation = match inst {
            R(val) => val,
            L(val) => 360 - val,
            _ => panic!("caso non previsto"),
        };

        for _i in 0..rotation / 90 {
            std::mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
            self.waypoint_y *= -1;
        }
    }

    fn forward(&mut self, amount: i32) {
        self.x += self.waypoint_x * amount;
        self.y += self.waypoint_y * amount;
    }

    fn exec(&mut self, inst: Instruction) {
        match inst {
            N(val) => {
                self.waypoint_y += val;
            }
            S(val) => {
                self.waypoint_y -= val;
            }
            W(val) => {
                self.waypoint_x -= val;
            }
            E(val) => {
                self.waypoint_x += val;
            }
            L(_val) | R(_val) => self.rotate(inst),
            F(val) => self.forward(val),
        }
    }
}

fn parseinput(line: &str) -> Instruction {
    let instr = line.as_bytes().get(0).unwrap();
    let num = line[1..].parse::<i32>().unwrap();
    return match *instr {
        b'N' => Instruction::N(num),
        b'S' => Instruction::S(num),
        b'W' => Instruction::W(num),
        b'E' => Instruction::E(num),
        b'L' => Instruction::L(num),
        b'R' => Instruction::R(num),
        b'F' => Instruction::F(num),
        _ => panic!("caso non previsto"),
    };
}

use aoc2020::utils;

fn main() {
    let lst = utils::read_list_parse("inputs/d12.txt", parseinput);

    let mut f = Ferry {
        facing: 90,
        x: 0,
        y: 0,
    };

    for i in lst.iter() {
        f.exec(*i);
    }

    println!("Part 1: {}", f.x.abs() + f.y.abs());

    let mut f2 = Ferry2 {
        waypoint_x: 10,
        waypoint_y: 1,
        x: 0,
        y: 0,
    };

    for i in lst.iter() {
        f2.exec(*i);
    }

    println!("Part 2: {}", f2.x.abs() + f2.y.abs());
}
