mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

use crate::day;

pub fn run(day: u32, part: day::Part) -> Option<day::Day> {
    let mut d = day::Day::new(2022, day);
    let success = match day {
        1 => d01::run(&mut d, part),
        2 => d02::run(&mut d, part),
        3 => d03::run(&mut d, part),
        4 => d04::run(&mut d, part),
        5 => d05::run(&mut d, part),
        6 => d06::run(&mut d, part),
        7 => d07::run(&mut d, part),
        8 => d08::run(&mut d, part),
        9 => d09::run(&mut d, part),
        10 => d10::run(&mut d, part),
        11 => d11::run(&mut d, part),
        12 => d12::run(&mut d, part),
        13 => d13::run(&mut d, part),
        14 => d14::run(&mut d, part),
        15 => d15::run(&mut d, part),
        16 => d16::run(&mut d, part),
        17 => d17::run(&mut d, part),
        18 => d18::run(&mut d, part),
        19 => d19::run(&mut d, part),
        20 => d20::run(&mut d, part),
        21 => d21::run(&mut d, part),
        22 => d22::run(&mut d, part),
        23 => d23::run(&mut d, part),
        24 => d24::run(&mut d, part),
        25 => d25::run(&mut d, part),
        _ => false,
    };
    if success {
        Option::Some(d)
    } else {
        Option::None
    }
}
