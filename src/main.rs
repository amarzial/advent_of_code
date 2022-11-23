extern crate argparse;

use aoc::run;
use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(Debug)]
struct Config {
    year: Option<u32>,
    day: Option<u32>,
    test: bool,
}

fn parse_args() -> Config {
    let mut y = 0;
    let mut d = 0;
    let mut test = false;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut y)
            .add_option(&["--year"], Store, "Run a specific year");
        ap.refer(&mut d)
            .add_option(&["--day"], Store, "Run a specific day");
        ap.refer(&mut test)
            .add_option(&["--test"], StoreTrue, "Run test input");
        ap.parse_args_or_exit();
    }

    let conf = Config {
        year: match y {
            0 => None,
            _ => Some(y),
        },
        day: match d {
            0 => None,
            _ => Some(d),
        },
        test: test,
    };
    conf
}

fn execute(cfg: &Config) {
    let years = match cfg.year {
        Some(y) => y..y + 1,
        None => 2018..2023,
    };

    let days = match cfg.day {
        Some(d) => d..d + 1,
        None => 1..26,
    };

    for year in years {
        let mut stars = 0;
        for day in days.clone() {
            match run(year, day, cfg.test) {
                Some(d) => {
                    d.print_result();
                    stars += d.stars();
                }
                None => {}
            }
        }
        print!("Year {:4} {}/50\n\n", year, stars);
    }
}

fn main() {
    let cfg = parse_args();
    execute(&cfg);
}
