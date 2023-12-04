use std::{cell::RefCell, collections::HashSet};
type Int = u64;
type NList = HashSet<u64>;

struct Card {
    winning_numbers: NList,
    numbers: NList,
}

impl Card {
    fn new(line: &str) -> Card {
        let parts = aoc::utils::read_pattern("Card {}: {} | {}", line).unwrap();

        let winning: NList = parts[1]
            .split(' ')
            .filter(|c| c != &"")
            .into_iter()
            .map(|n| n.parse::<Int>().unwrap())
            .collect();
        let numbers: NList = parts[2]
            .split(' ')
            .filter(|c| c != &"")
            .into_iter()
            .map(|n| n.parse::<Int>().unwrap())
            .collect();

        Card {
            winning_numbers: winning,
            numbers: numbers,
        }
    }

    fn matches(&self) -> Int {
        let int = self.winning_numbers.intersection(&self.numbers);
        let res: HashSet<_> = int.collect();
        res.len() as Int
    }

    fn score(&self) -> Int {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            return Int::pow(2, matches as u32 - 1);
        }
    }
}

fn part_one(input: &str) -> Option<Int> {
    let cards = aoc::utils::read_list_parse(input, Card::new);

    let total = cards.iter().map(|c| c.score()).sum();
    Some(total)
}

fn part_two(input: &str) -> Option<Int> {
    let cards = aoc::utils::read_list_parse(input, |l| RefCell::new(Card::new(l)));
    let mut counts = vec![];
    counts.resize(cards.len(), 1 as Int);

    for c in 0..cards.len() {
        let matches = cards[c].borrow().matches();
        if matches > 0 {
            let lower = c + 1;
            let upper = c + matches as usize;
            for c_idx in lower..=upper {
                counts[c_idx] += counts[c];
            }
        }
    }
    Some(counts.iter().sum())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 04);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 04);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 04);
        assert_eq!(part_two(&input), Some(30));
    }
}
