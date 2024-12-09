fn parse_input(input: &str) -> Vec<i32> {
    let mut res = Vec::new();
    let mut id = 0;
    for ch in input.lines().next().unwrap().chars().enumerate() {
        let mut value = -1;
        if ch.0 % 2 == 0 {
            value = id;
            id += 1;
        }
        let count = ch.1.to_digit(10).unwrap();
        res.resize(res.len() + count as usize, value);
    }
    res
}

fn calc_score(mem: &[i32]) -> usize {
    let mut total = 0;
    for i in 0..mem.len() {
        if mem[i] == -1 {
            continue;
        }
        total += (mem[i] as usize) * i;
    }
    total
}

fn part_one(input: &str) -> Option<usize> {
    let mut mem = parse_input(input);
    let mut lstart = 0;
    let mut rstart = mem.len() - 1;
    loop {
        while mem[rstart] == -1 && rstart > 0 {
            rstart -= 1;
        }

        while mem[lstart] != -1 && lstart < mem.len() - 1 {
            lstart += 1;
        }

        if lstart >= rstart {
            break;
        }

        mem.swap(lstart, rstart);
        rstart -= 1;
    }

    let total = calc_score(&mem);
    Some(total)
}

fn next_chunk(mem: &[i32], mut start: usize) -> (usize, usize) {
    while start > 0 && mem[start] == -1 {
        start -= 1;
    }

    if mem[start] != -1 {
        let c = mem[start];
        let mut cursor = 0;
        while (start - cursor > 0) && (mem[start - cursor] == c) {
            cursor += 1;
        }
        return (start - cursor + 1, cursor);
    }
    (0, 0)
}

fn find_free_spaces(mem: &[i32]) -> Vec<(usize, usize)> {
    let mut spaces = vec![];

    let mut count = 0;
    for c in mem.iter().enumerate() {
        if *c.1 != -1 {
            if count > 0 || c.0 == (mem.len() - 1) {
                spaces.push((c.0 - count, count));
            }
            count = 0;
        } else {
            count += 1;
        }
    }
    spaces
}

fn _print_mem(mem: &[i32]) {
    for n in mem {
        if *n == -1 {
            print!(".");
        } else {
            print!("{}", n)
        }
    }
    println!();
}

fn part_two(input: &str) -> Option<usize> {
    let mut mem = parse_input(input);
    let mut rstart = mem.len() - 1;

    let mut free_spaces = find_free_spaces(&mem);

    loop {
        let rnext = next_chunk(&mem, rstart);
        let found_free = free_spaces.iter().position(|e| e.1 >= rnext.1);

        if rnext.1 == 0 {
            break;
        }

        if found_free.is_none() {
            rstart = rnext.0 - 1;
            continue;
        }
        let lnext = free_spaces[found_free.unwrap()];

        if lnext.0 >= rnext.0 {
            rstart = rnext.0 - 1;
            continue;
        }

        for i in 0..rnext.1 {
            mem.swap(rnext.0 + i, lnext.0 + i);
        }

        if lnext.1 > rnext.1 {
            free_spaces[found_free.unwrap()] = (lnext.0 + rnext.1, lnext.1 - rnext.1);
        } else {
            free_spaces.remove(found_free.unwrap());
        }

        rstart = rnext.0 - 1;
    }

    let total = calc_score(&mem);
    Some(total)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 09);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 09);
        assert_eq!(part_one(&input), Some(1928));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 09);
        assert_eq!(part_two(&input), Some(2858));
    }
}
