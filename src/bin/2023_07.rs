use std::cmp::Ordering;

type Int = i32;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: Int,
}

fn charval(card: char) -> Int {
    match card {
        '2'..='9' => (card.to_digit(10).unwrap() as Int) - 2,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}

fn charval2(card: char) -> Int {
    match card {
        'J' => 0,
        '2'..='9' => (card.to_digit(10).unwrap() as Int) - 1,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}

impl Hand {
    fn strength(&self) -> Int {
        let mut table = [0; 13];
        for card in self.cards.chars() {
            let val = charval(card);
            table[val as usize] += 1;
        }
        let mut pairs = 0;
        let mut threes = 0;
        for count in table {
            if count == 5 {
                return 7;
            } else if count == 4 {
                return 6;
            } else if count == 3 {
                threes += 1;
            } else if count == 2 {
                pairs += 1;
            }
        }
        if threes == 1 && pairs == 1 {
            return 5;
        } else if threes == 1 && pairs == 0 {
            return 4;
        } else if pairs == 2 {
            return 3;
        } else if pairs == 1 {
            return 2;
        }
        1
    }

    fn strength2(&self) -> Int {
        let mut table = [0; 13];
        for card in self.cards.chars() {
            let val = charval2(card);
            table[val as usize] += 1;
        }

        let jokers = table[0];
        table[0] = 0;
        let mut best = 0;
        let mut second = 0;
        for count in table {
            if best < count {
                second = best;
                best = count;
            } else {
                second = second.max(count);
            }
        }
        let strength = match (best + jokers, second) {
            (5, _) => 7,
            (4, _) => 6,
            (3, 2) => 5,
            (3, _) => 4,
            (2, 2) => 3,
            (2, _) => 2,
            _ => 1,
        };
        strength
    }

    fn compare(&self, other: &Hand) -> Ordering {
        let best = self.strength().cmp(&other.strength());
        match best {
            Ordering::Equal => {
                let mut cmp = Ordering::Equal;
                let mut it_a = self.cards.chars();
                let mut it_b = other.cards.chars();
                loop {
                    let a = it_a.next();
                    let b = it_b.next();
                    if a.is_none() {
                        break;
                    }
                    cmp = charval(a.unwrap()).cmp(&charval(b.unwrap()));
                    match cmp {
                        Ordering::Equal => {}
                        _ => break,
                    }
                }
                cmp
            }
            _ => best,
        }
    }

    fn compare2(&self, other: &Hand) -> Ordering {
        let best = self.strength2().cmp(&other.strength2());
        match best {
            Ordering::Equal => {
                let mut cmp = Ordering::Equal;
                let mut it_a = self.cards.chars();
                let mut it_b = other.cards.chars();
                loop {
                    let a = it_a.next();
                    let b = it_b.next();
                    if a.is_none() {
                        break;
                    }
                    cmp = charval2(a.unwrap()).cmp(&charval2(b.unwrap()));
                    match cmp {
                        Ordering::Equal => {}
                        _ => break,
                    }
                }
                cmp
            }
            _ => best,
        }
    }
}

fn part_one(input: &str) -> Option<Int> {
    let mut hands = aoc::utils::read_list_parse(input, |l| {
        let mut parts = l.split(' ');
        let cards = String::from(parts.next().unwrap());
        let bid = parts.next().unwrap().parse::<Int>().unwrap();
        Hand { cards, bid }
    });

    hands.sort_by(|a, b| a.compare(b));

    let tot: Int = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as Int * h.bid)
        .sum();
    Some(tot)
}

fn part_two(input: &str) -> Option<Int> {
    let mut hands = aoc::utils::read_list_parse(input, |l| {
        let mut parts = l.split(' ');
        let cards = String::from(parts.next().unwrap());
        let bid = parts.next().unwrap().parse::<Int>().unwrap();
        Hand { cards, bid }
    });

    hands.sort_by(|a, b| a.compare2(b));

    let tot: Int = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as Int * h.bid)
        .sum();
    Some(tot)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 07);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 07);
        assert_eq!(part_one(&input), Some(6440));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 07);
        assert_eq!(part_two(&input), Some(5905));
    }
}
