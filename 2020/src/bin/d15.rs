use aoc2020::utils;

fn main() {
    let lst: Vec<String> = utils::read_list("inputs/d15.txt");
    let input: Vec<i32> = lst
        .get(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut numbers = vec![0; 3e7 as usize];

    let mut current = 1;
    let mut next = 0;
    for n in input.iter() {
        numbers[*n as usize] = current;
        current += 1;
    }

    while current < 2020 {
        let first = numbers[next] == 0;
        let n = match first {
            true => 0,
            false => current - numbers[next],
        };
        numbers[next] = current;
        current += 1;
        next = n;
    }

    println!("Part 1: {}", next);

    while current < 30000000 {
        let first = numbers[next] == 0;
        let n = match first {
            true => 0,
            false => current - numbers[next],
        };
        numbers[next] = current;
        current += 1;
        next = n;
    }
    println!("Part 2: {}", next);
}
