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
