fn bank_max_jolts(bank: &str, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    let max = bank[..bank.len() - (count - 1)].chars().max().unwrap();
    let idx = bank.find(max).unwrap();
    let mut out = max.to_digit(10).unwrap() as usize;

    if count > 1 {
        let prev = bank_max_jolts(&bank[idx + 1..], count - 1);
        out *= (10 as u64).pow(((prev as f64).log10().floor() + 1.) as u32) as usize;
        out += prev;
    }

    out
}

fn part_one(input: &str) -> Option<String> {
    let banks = input.lines().collect::<Vec<&str>>();

    let total = banks.iter().fold(0,|acc, &bank| {
        let max_jolts = bank_max_jolts(bank, 2);
        acc + max_jolts
    });
    Some(total.to_string())
}

fn part_two(input: &str) -> Option<String> {
    let banks = input.lines().collect::<Vec<&str>>();

    let total = banks.iter().fold(0,|acc, &bank| {
        let max_jolts = bank_max_jolts(bank, 12);
        acc + max_jolts
    });
    Some(total.to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2025, 03);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2025, 03);
        assert_eq!(part_one(&input), Some("357".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2025, 03);
        assert_eq!(part_two(&input), Some("3121910778619".to_string()));
    }
}
