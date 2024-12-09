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

fn next_free(mem: &[i32], mut start: usize) -> (usize, usize) {
    while start < (mem.len() - 1) && mem[start] != -1 {
        start += 1;
    }

    if start >= mem.len() {
        return (0, 0);
    }

    if mem[start] == -1 {
        let c = mem[start];
        let mut cursor = 0;
        while (start + cursor < mem.len()) && (mem[start + cursor] == c) {
            cursor += 1;
        }
        return (start, cursor);
    }
    (0, 0)
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

fn part_two(input: &str) -> Option<usize> {
    let mut mem = parse_input(input);
    let mut rstart = mem.len() - 1;

    loop {
        let rnext = next_chunk(&mem, rstart);
        let mut lnext = next_free(&mem, 0);

        if rnext.1 == 0 {
            break;
        }

        while lnext.1 > 0 && lnext.0 < mem.len() && lnext.1 < rnext.1 {
            lnext = next_free(&mem, lnext.0 + lnext.1);
        }

        if lnext.1 == 0 || lnext.0 >= rnext.0 {
            rstart = rnext.0 - 1;
            continue;
        }

        for i in 0..rnext.1 {
            mem.swap(rnext.0 + i, lnext.0 + i);
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
