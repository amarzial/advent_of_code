pub fn load_input(folder: &str, year: i32, day: i32) -> String {
    let path = format!("./{}/{:04}_{:02}.txt", folder, year, day);
    std::fs::read_to_string(&path).expect(&path)
}

pub fn read_list<T: std::str::FromStr + std::fmt::Debug>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut list: Vec<T> = Vec::new();

    for line in input.lines() {
        list.push(line.parse().unwrap());
    }
    return list;
}

pub fn read_list_parse<T>(input: &str, func: fn(&str) -> T) -> Vec<T> {
    let mut list: Vec<T> = Vec::new();

    for line in input.lines() {
        list.push(func(&line));
    }
    return list;
}

pub fn read_line<T>(input: &str, separator: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let data: Vec<T> = input[0..input.len() - 1]
        .split(separator)
        .map(|x| return x.parse().unwrap())
        .collect();
    return data;
}

pub fn read_pattern<'a>(pattern: &'a str, input: &'a str) -> Option<Vec<&'a str>> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut end;

    for m in pattern.split("{}") {
        match input[start..].find(m) {
            Some(idx) => {
                // println!("{}", &input[start..]);
                end = start + idx;
                if end > start {
                    out.push(&input[start..end]);
                }
                start = start + idx + m.len();
                // println!("{}- {}, {}", idx, start, end);
            }
            None => {
                return None;
            }
        }
    }

    if start != input.len() {
        out.push(&input[start..]);
    }
    Some(out)
}

pub fn cache(enter: Option<String>) -> Option<String> {
    static mut C: Option<String> = None;

    let mut current = None;
    unsafe {
        if C.is_some() {
            current = C.clone();
        }
        C = enter;
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pattern_parser() {
        let res = read_pattern("move {} from {} to {}", "move 1 from 32 to 4");
        assert_eq!(res, Some(vec!["1", "32", "4"]));
        let res = read_pattern("{}-{},{}-{}", "1-2,30-4");
        assert_eq!(res, Some(vec!["1", "2", "30", "4"]));
        let res = read_pattern("move {} from {} to {} asdf", "move 1 from 32 to 4 asdf");
        assert_eq!(res, Some(vec!["1", "32", "4"]));
        let res = read_pattern("move {} from {} to {} asdf", "movasde 1 frofm 32 to 4 asdf");
        assert_eq!(res, None);
    }
}
