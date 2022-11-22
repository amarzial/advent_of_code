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
            match run(year, day) {
                Some(d) => {
                    d.print_result();
                }
                None => {}
            }
        }
    }
}
