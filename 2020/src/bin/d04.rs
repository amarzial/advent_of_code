use aoc2020::utils;

type Passport = std::collections::HashMap<String, String>;

fn fill_data(pass: &mut Passport, line: &str) {
    for item in line.split_ascii_whitespace() {
        let info = item.split(':').collect::<Vec<_>>();
        pass.insert(String::from(info[0]), String::from(info[1]));
    }
}

fn read_passport(iter: &mut std::slice::Iter<String>) -> Option<Passport> {
    let mut pass = Passport::new();
    loop {
        match iter.next() {
            Some(line) => {
                if line.is_empty() {
                    return Some(pass);
                }
                fill_data(&mut pass, line)
            }
            _ => return if !pass.is_empty() { Some(pass) } else { None },
        }
    }
}

fn validate1(p: &Passport, params: &[&str]) -> bool {
    let mut valid = true;
    for param in params {
        if !p.contains_key(*param) {
            valid = false;
            break;
        }
    }

    return valid;
}

fn check_year(year: &str, min: u32, max: u32) -> bool {
    if year.len() != 4 {
        return false;
    }

    let num: u32 = year.parse().unwrap_or(0);
    return num >= min && num <= max;
}

fn check_height(height: &str) -> bool {
    let unit = if height.ends_with("cm") {
        "cm"
    } else {
        if height.ends_with("in") {
            "in"
        } else {
            return false;
        }
    };
    let value = std::str::from_utf8(&height.as_bytes()[0..height.find(unit).unwrap()])
        .unwrap()
        .parse::<u32>()
        .unwrap();
    if height.ends_with("cm") && (value >= 150) && (value <= 193) {
        return true;
    } else if height.ends_with("in") && (value >= 59) && (value <= 76) {
        return true;
    }
    return false;
}

fn check_hair(color: &str) -> bool {
    if !color.starts_with('#') || (color.len() != 7) {
        return false;
    }

    for i in 1..color.as_bytes().len() {
        let c = color.as_bytes()[i];
        if !(c >= b'0' && c <= b'9') && !(c >= b'a' && c <= b'f') {
            return false;
        }
    }
    return true;
}

fn check_pid(pid: &str) -> bool {
    if pid.len() != 9 {
        return false;
    }

    for c in pid.as_bytes() {
        if !(*c >= b'0' && *c <= b'9') {
            return false;
        }
    }
    return true;
}

fn validate2(p: &Passport, eyes: &std::collections::HashSet<String>) -> bool {
    let mut valid = check_year(p.get("byr").unwrap(), 1920, 2002);
    if !valid {
        return false;
    }
    valid = check_year(p.get("iyr").unwrap(), 2010, 2020);
    if !valid {
        return false;
    }
    valid = check_year(p.get("eyr").unwrap(), 2020, 2030);
    if !valid {
        return false;
    }
    valid = check_height(p.get("hgt").unwrap());
    if !valid {
        return false;
    }
    valid = check_hair(p.get("hcl").unwrap());
    if !valid {
        return false;
    }
    valid = eyes.contains(p.get("ecl").unwrap());
    if !valid {
        return false;
    }
    valid = check_pid(p.get("pid").unwrap());
    if !valid {
        return false;
    }
    return true;
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or(String::from("inputs/d04.txt"));
    let list = utils::read_list::<String>(&filename);

    let valid_fields1 = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let eyes: std::collections::HashSet<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|&s| {
            return String::from(s);
        })
        .collect();

    let mut lines = list.iter();

    let mut count = 0;
    let mut count2 = 0;
    while let Some(pass) = read_passport(&mut lines) {
        let valid = validate1(&pass, &valid_fields1);
        let valid2 = if valid {
            validate2(&pass, &eyes)
        } else {
            false
        };
        count += if valid { 1 } else { 0 };
        count2 += if valid && valid2 { 1 } else { 0 };
    }
    println!("Part 1: {}", count);
    println!("Part 2: {}", count2);
}
