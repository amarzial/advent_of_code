use aoc::utils::{self, cache};

fn part_one(input: &str) -> Option<String> {
    let sonar: Vec<u32> = utils::read_list(input);

    let mut last = *sonar.get(0).unwrap();
    let mut cnt = 0;
    for depth in sonar.iter() {
        if *depth > last {
            cnt += 1;
        }
        last = *depth;
    }

    let mut cnt2 = 0;
    let mut last2: u32 = sonar.windows(3).next().unwrap().iter().sum();
    for win in sonar.windows(3).skip(1) {
        let s: u32 = win.iter().sum();
        if s > last2 {
            cnt2 += 1;
        }
        last2 = s;
    }

    let p1 = cnt.to_string();
    let p2 = cnt2.to_string();
    utils::cache(Some(p2));
    Some(p1)
}

fn part_two(_input: &str) -> Option<String> {
    cache(None)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2021, 01);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2021, 01);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2021, 01);
        assert_eq!(part_two(&input), None);
    }
}
