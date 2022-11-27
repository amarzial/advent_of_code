use std::{fs::File, fs::OpenOptions, io::Write};

const DAY_TEMPLATE: &str = r#"fn part_one(input: &str) -> Option<String> {
    None
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", YEAR, DAY);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", YEAR, DAY);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", YEAR, DAY);
        assert_eq!(part_two(&input), None);
    }
}
"#;

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn main() {
    let y = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    let d = std::env::args().nth(2).unwrap().parse::<u32>().unwrap();

    let fmt_year = format!("{:04}", y);
    let fmt_day = format!("{:02}", d);

    {
        match create_file(&format!("./src/bin/{}_{}.rs", fmt_year, fmt_day)) {
            Ok(mut file) => {
                file.write_all(
                    DAY_TEMPLATE
                        .replace("YEAR", &fmt_year)
                        .replace("DAY", &fmt_day)
                        .as_bytes(),
                )
                .unwrap();
                println!("Source file created")
            }
            Err(_) => println!("Source file already found"),
        }
    }

    {
        match create_file(&format!("inputs/{}_{}.txt", fmt_year, fmt_day)) {
            Ok(_) => println!("Input created"),
            Err(_) => println!("Input already found"),
        }
    }

    {
        match create_file(&format!("examples/{}_{}.txt", fmt_year, fmt_day)) {
            Ok(_) => println!("Input created"),
            Err(_) => println!("Input already found"),
        }
    }
}
