use std::io::BufRead;

pub fn read_int_list(path: &str) -> Vec<u32> {
    let f = std::fs::File::open(path).unwrap();
    let mut buff = std::io::BufReader::new(f);
    let mut list: Vec<u32> = Vec::new();
    let mut s = String::new();
    while buff.read_line(&mut s).unwrap() > 0 {
        s.pop();
        list.push(s.parse().unwrap());
        s.clear();
    }
    return list;
}

pub fn read_list<T>(path: &str, func: fn(&str) -> T) -> Vec<T> {
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
