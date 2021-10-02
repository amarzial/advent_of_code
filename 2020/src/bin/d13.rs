use aoc2020::utils;

fn find_common(start: u64, current: u64, increment: u64, offset: u64) -> u64 {
    let mut times = 1;
    while (start + (increment * times) + offset) % current != 0 {
        times += 1;
    }
    return start + (increment * times);
}

fn main() {
    let input: Vec<String> = utils::read_list("inputs/d13.txt");
    let timestamp: u64 = input.get(0).unwrap().parse().unwrap();
    let departures: Vec<u64> = input
        .get(1)
        .unwrap()
        .split(',')
        .map(|el| el.parse().unwrap_or(0))
        .collect();

    let mut closest = u64::MAX;
    let mut line = 0;
    for bus in departures.iter().filter(|el| *el > &0) {
        let offset = (timestamp / *bus) * *bus + *bus - timestamp;
        if offset < closest {
            closest = offset;
            line = *bus;
        }
    }

    println!("Part 1: {}", closest * line);

    let mut start = 0;
    let mut increment = 1;
    let mut out = 0;

    for i in 0..departures.len() {
        let bus = departures.get(i).unwrap();
        if *bus == 0 {
            continue;
        }
        let n = find_common(start, *bus, increment, i as u64);
        increment *= *bus;
        start = n;
        out = n;
    }

    println!("Part 2: {}", out);
}
