#[derive(Clone, Copy)]
enum Instruction {
    Nop,
    Addx(i32),
}

struct CPU {
    register: i32,
    cycle: i32,
    busy: usize,
    current: Instruction,
}

fn part_one(input: &str) -> Option<i32> {
    let instructions = aoc::utils::read_list_parse(input, |l| {
        let mut inst = l.split_whitespace();
        match inst.next().unwrap() {
            "noop" => Instruction::Nop,
            "addx" => Instruction::Addx(inst.next().unwrap().parse().unwrap()),
            _ => Instruction::Nop,
        }
    });

    let mut cpu = CPU {
        register: 1,
        cycle: 0,
        busy: 0,
        current: Instruction::Nop,
    };

    let steps = vec![20, 60, 100, 140, 180, 220];
    let mut signal = 0;

    for i in instructions {
        match i {
            Instruction::Nop => {
                cpu.current = Instruction::Nop;
                cpu.busy = 1
            }
            Instruction::Addx(_) => {
                cpu.current = i;
                cpu.busy = 2
            }
        }

        while cpu.busy != 0 {
            if steps.contains(&(cpu.cycle + 1)) {
                signal += (cpu.cycle + 1) * cpu.register;
            }
            cpu.cycle += 1;
            cpu.busy -= 1;
        }
        match i {
            Instruction::Addx(n) => {
                cpu.register += n;
            }
            Instruction::Nop => {}
        }
    }

    Some(signal)
}

fn print(crt: Vec<i32>, width: usize, height: usize) -> String {
    let mut out = String::new();
    for y in 0..height {
        for x in 0..width {
            out += if crt[y * width + x] == 0 { " " } else { "#" };
        }
        out += "\n";
    }
    out
}

fn part_two(input: &str) -> Option<String> {
    let instructions = aoc::utils::read_list_parse(input, |l| {
        let mut inst = l.split_whitespace();
        match inst.next().unwrap() {
            "noop" => Instruction::Nop,
            "addx" => Instruction::Addx(inst.next().unwrap().parse().unwrap()),
            _ => Instruction::Nop,
        }
    });

    let mut cpu = CPU {
        register: 1,
        cycle: 0,
        busy: 0,
        current: Instruction::Nop,
    };

    let width = 40;
    let height = 6;
    let mut crt = Vec::new();
    crt.resize(width * height, 0);

    for i in instructions {
        match i {
            Instruction::Nop => {
                cpu.current = Instruction::Nop;
                cpu.busy = 1
            }
            Instruction::Addx(_) => {
                cpu.current = i;
                cpu.busy = 2
            }
        }

        while cpu.busy != 0 {
            crt[cpu.cycle as usize] = if cpu.register.abs_diff(cpu.cycle % width as i32) < 2 {
                1
            } else {
                0
            };
            cpu.cycle += 1;
            cpu.busy -= 1;
        }
        match i {
            Instruction::Addx(n) => {
                cpu.register += n;
            }
            Instruction::Nop => {}
        }
    }

    Some(String::from("\n") + &print(crt, width, height))
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 10);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 10);
        assert_eq!(part_one(&input), Some(13140));
    }
}
