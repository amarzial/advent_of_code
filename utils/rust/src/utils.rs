use std::io::BufRead;

pub fn get_input() -> String {
    match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            let mut file = String::from("input/");
            file += std::env::args().nth(0).unwrap().split("/").last().unwrap();
            file += ".txt";
            file
        }
    }
}

pub fn read_list<T: std::str::FromStr + std::fmt::Debug>(path: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = std::fs::File::open(path).unwrap();
    let mut buff = std::io::BufReader::new(f);
    let mut list: Vec<T> = Vec::new();
    let mut s = String::new();
    while buff.read_line(&mut s).unwrap() > 0 {
        s.pop();
        list.push(s.parse().unwrap());
        s.clear();
    }
    return list;
}

pub fn read_list_parse<T>(path: &str, func: fn(&str) -> T) -> Vec<T> {
    let f = std::fs::File::open(path).unwrap();
    let mut buff = std::io::BufReader::new(f);
    let mut list: Vec<T> = Vec::new();
    let mut s = String::new();
    while buff.read_line(&mut s).unwrap() > 0 {
        s.pop();
        list.push(func(&s));
        s.clear();
    }
    return list;
}

pub fn read_line<T>(path: &str, separator: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let input = std::fs::read_to_string(path).unwrap();
    let data: Vec<T> = input[0..input.len() - 1]
        .split(separator)
        .map(|x| return x.parse().unwrap())
        .collect();
    return data;
}
