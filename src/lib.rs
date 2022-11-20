pub mod day;
pub mod utils;
mod y2021;
mod y2022;

pub fn run(year: u32, day: u32) -> Option<day::Day> {
    match year {
        2021 => y2021::run(day, day::Part::Both),
        2022 => y2022::run(day, day::Part::Both),
        _ => Option::None,
    }
}
