use std::process::Command;

fn parse_time(val: &str, postfix: &str) -> f64 {
    val.split(postfix).next().unwrap().parse().unwrap()
}

pub fn parse_exec_time(output: &str) -> f64 {
    output.lines().fold(0_f64, |acc, l| {
        if !l.contains("time:") {
            acc
        } else {
            let timing = l.split("(time: ").last().unwrap();
            if timing.contains("ns)") {
                acc
            } else if timing.contains("µs)") {
                acc + parse_time(timing, "µs") / 1000_f64
            } else if timing.contains("ms)") {
                acc + parse_time(timing, "ms")
            } else if timing.contains("s)") {
                acc + parse_time(timing, "s") * 1000_f64
            } else {
                acc
            }
        }
    })
}

fn run_day(year: i32, day: i32) -> f64 {
    let bin = format!("{:04}_{:02}", year, day);

    let mut args = vec!["run", "--bin", &bin];
    if cfg!(not(debug_assertions)) {
        args.push("--release");
    }

    let cmd = Command::new("cargo").args(&args).output().unwrap();
    if cmd.stdout.is_empty() {
        0_f64
    } else {
        let t = parse_exec_time(&String::from_utf8(cmd.stdout).unwrap());
        println!("Year {} - Day {:02}: {:.3}ms", year, day, t);
        t
    }
}

fn main() {
    let mut total = 0_f64;
    for y in 2015..=2022 {
        for d in 1..=25 {
            total += run_day(y, d);
        }
    }

    println!("Total time: {:.2}ms", total);
}
