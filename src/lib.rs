pub mod day;
pub mod utils;
mod y2022;

pub fn run(year: u32, day: u8) -> Option<day::Day> {
    match year {
        2022 => y2022::run(day, day::Part::Both),
        _ => Option::None,
    }
}
