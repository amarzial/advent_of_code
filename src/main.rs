use aoc::day::Day;
use aoc::day::Part;
use aoc::run;

#[derive(Debug)]
struct Config {
    year: Option<u32>,
    day: Option<u32>,
}

fn parse_args() -> Config {
    let mut args = std::env::args();
    args.next();

    let y = match args.next() {
        Some(s) => Some(s.parse::<u32>().unwrap()),
        _ => None,
    };

    let d = match args.next() {
        Some(s) => Some(s.parse::<u32>().unwrap()),
        _ => None,
    };

    let conf = Config { year: y, day: d };
    conf
}

fn display_result(day: &Day) {
    let p1 = match &day.part1 {
        Some(s) => s.to_owned(),
        None => String::from(""),
    };
    let p2 = match &day.part2 {
        Some(s) => s.to_owned(),
        None => String::from(""),
    };

    if !day.part1.is_some() && !day.part2.is_some() {
        print!("Year {}, Day {}: Missing\n", day.year, day.day);
    } else {
        print!(
            "Year {}, Day {}: {}.{:06}s\n  part 1: {}\n  part 2: {}\n",
            day.year,
            day.day,
            day.time.as_secs(),
            day.time.subsec_micros(),
            p1,
            p2
        );
    }
}

fn main() {
    let cfg = parse_args();
    let years = match cfg.year {
        Some(y) => y..y + 1,
        None => 2021..2023,
    };

    let days = match cfg.day {
        Some(d) => d..d + 1,
        None => 1..26,
    };

    for year in years {
        for day in days.clone() {
            match run(year, day, Part::Both) {
                Some(d) => {
                    display_result(&d);
                }
                None => {}
            }
        }
    }
}
