pub mod day;
pub mod utils;
mod y2021;
mod y2022;

use std::time::Instant;

pub fn run(year: u32, day: u32, part: day::Part) -> Option<day::Day> {
    let mut d = day::Day::new(year, day);
    let now = Instant::now();
    let success = match year {
        2021 => match day {
            1 => y2021::d01::run(&mut d, part),
            2 => y2021::d02::run(&mut d, part),
            3 => y2021::d03::run(&mut d, part),
            4 => y2021::d04::run(&mut d, part),
            5 => y2021::d05::run(&mut d, part),
            6 => y2021::d06::run(&mut d, part),
            7 => y2021::d07::run(&mut d, part),
            8 => y2021::d08::run(&mut d, part),
            9 => y2021::d09::run(&mut d, part),
            10 => y2021::d10::run(&mut d, part),
            11 => y2021::d11::run(&mut d, part),
            12 => y2021::d12::run(&mut d, part),
            13 => y2021::d13::run(&mut d, part),
            14 => y2021::d14::run(&mut d, part),
            15 => y2021::d15::run(&mut d, part),
            16 => y2021::d16::run(&mut d, part),
            17 => y2021::d17::run(&mut d, part),
            18 => y2021::d18::run(&mut d, part),
            19 => y2021::d19::run(&mut d, part),
            20 => y2021::d20::run(&mut d, part),
            21 => y2021::d21::run(&mut d, part),
            22 => y2021::d22::run(&mut d, part),
            23 => y2021::d23::run(&mut d, part),
            24 => y2021::d24::run(&mut d, part),
            25 => y2021::d25::run(&mut d, part),
            _ => false,
        },
        2022 => match day {
            1 => y2022::d01::run(&mut d, part),
            2 => y2022::d02::run(&mut d, part),
            3 => y2022::d03::run(&mut d, part),
            4 => y2022::d04::run(&mut d, part),
            5 => y2022::d05::run(&mut d, part),
            6 => y2022::d06::run(&mut d, part),
            7 => y2022::d07::run(&mut d, part),
            8 => y2022::d08::run(&mut d, part),
            9 => y2022::d09::run(&mut d, part),
            10 => y2022::d10::run(&mut d, part),
            11 => y2022::d11::run(&mut d, part),
            12 => y2022::d12::run(&mut d, part),
            13 => y2022::d13::run(&mut d, part),
            14 => y2022::d14::run(&mut d, part),
            15 => y2022::d15::run(&mut d, part),
            16 => y2022::d16::run(&mut d, part),
            17 => y2022::d17::run(&mut d, part),
            18 => y2022::d18::run(&mut d, part),
            19 => y2022::d19::run(&mut d, part),
            20 => y2022::d20::run(&mut d, part),
            21 => y2022::d21::run(&mut d, part),
            22 => y2022::d22::run(&mut d, part),
            23 => y2022::d23::run(&mut d, part),
            24 => y2022::d24::run(&mut d, part),
            25 => y2022::d25::run(&mut d, part),
            _ => false,
        },
        _ => false,
    };
    d.time = now.elapsed();
    if success {
        Option::Some(d)
    } else {
        Option::None
    }
}
