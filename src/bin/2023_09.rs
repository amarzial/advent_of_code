use std::vec;

type Sequence = Vec<i32>;

fn reduce(seq: &Sequence) -> (Sequence, bool) {
    let mut new = vec![];
    new.reserve(seq.len() - 1);
    let mut it = seq.iter();

    let mut prev = it.next().unwrap();
    let mut all_zero = true;
    for v in it {
        let diff = v - prev;
        new.push(diff);
        prev = v;
        if diff != 0 {
            all_zero = false;
        }
    }
    (new, all_zero)
}

fn part_one(input: &str) -> Option<i64> {
    let sequences: Vec<Sequence> = aoc::utils::read_list_parse(input, |l| {
        l.split(' ').map(|v| v.parse().unwrap()).collect()
    });

    let mut tot = 0;
    for seq in sequences {
        let mut s = seq.clone();
        let mut increments = vec![];
        increments.push(s.last().unwrap().clone());
        loop {
            let res = reduce(&s);
            let last = res.0.last().unwrap().clone();
            increments.push(last);
            if res.1 {
                break;
            }
            s = res.0;
        }
        let mut it = increments.iter().rev();

        let mut inc = *it.next().unwrap();
        for val in it {
            inc += val;
        }
        tot += inc as i64;
    }
    Some(tot)
}

fn part_two(input: &str) -> Option<i64> {
    let sequences: Vec<Sequence> = aoc::utils::read_list_parse(input, |l| {
        l.split(' ').map(|v| v.parse().unwrap()).collect()
    });

    let mut tot = 0;
    for seq in sequences {
        let mut s = seq.clone();
        let mut increments = vec![];
        increments.push(s.first().unwrap().clone());
        loop {
            let res = reduce(&s);
            let last = res.0.first().unwrap().clone();
            increments.push(last);
            if res.1 {
                break;
            }
            s = res.0;
        }

        let mut it = increments.iter().rev();

        let mut inc = *it.next().unwrap();
        for val in it {
            inc = val - inc;
        }
        tot += inc as i64;
    }
    Some(tot)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 09);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 09);
        assert_eq!(part_one(&input), Some(114));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 09);
        assert_eq!(part_two(&input), Some(2));
    }
}
