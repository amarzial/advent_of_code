use aoc::run;
use aoc::day::Day;

#[derive(Debug)]
struct Config {
    year: Option<u32>,
    day: Option<u32>
}

fn parse_args() -> Config {
    let mut args = std::env::args();
    args.next();
   
    let y = match args.next() {
        Some(s) => Some(s.parse::<u32>().unwrap()),
        _ => None
    };


    let d = match args.next() {
        Some(s) => Some(s.parse::<u32>().unwrap()),
        _ => None
    };

    let conf = Config {year: y, day: d};
    conf
}

fn display_result(day: &Day) {
    let p1 = match &day.part1 {
        Some(s) => s.to_owned(),
        None => String::from("")
    };
    let p2 = match &day.part2 {
        Some(s) => s.to_owned(),
        None => String::from("")
    };
    print!("Year {}, Day {}\n  part 1: {}\n  part 2: {}\n",day.year, day.day, p1, p2);
}

fn main() {
    let cfg = parse_args();

    for year in 2021..2022 {
        for day in 1..25 {
            match run(year, day) {
                Some(d) => { display_result(&d); },
                None =>  {}
            }
        }
    }
}
