use aoc2020::utils;

fn check_number(haystack: &[i64], needle: i64) -> bool {
    for i in 0..haystack.len() {
        for j in i..haystack.len() {
            if haystack.get(i).unwrap() + haystack.get(j).unwrap() == needle {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let list: Vec<i64> = utils::read_list("inputs/d09.txt");

    let mut number = 0;
    let mut sums = list.windows(25);
    for i in 25..list.len() {
        let sum = sums.next().unwrap();
        if !check_number(sum, *list.get(i).unwrap()) {
            number = *list.get(i).unwrap();
            println!("Part 1: {}", number);
            break;
        }
    }

    let mut lower = 0;
    let mut upper = 0;
    let mut sum = 0;
    for i in 0..list.len() {
        let num = list.get(i).unwrap();
        sum += num;
        upper = i;
        while sum > number {
            sum -= list.get(lower).unwrap();
            lower += 1;
        }
        if sum == number {
            break;
        }
    }
    let mut max = 0;
    let mut min = std::i64::MAX;
    for e in list.get(lower..=upper).unwrap() {
        max = std::cmp::max(*e, max);
        min = std::cmp::min(*e, min);
    }
    println!("Part 2: {}", min + max);
}
