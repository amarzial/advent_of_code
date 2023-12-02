use std::collections::HashMap;

type Cube = (u32, String);
type Set = Vec<Cube>;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn valid(&self, total: &Set) -> bool {
        for set in &self.sets {
            for c in set {
                for res in total {
                    if (c.1 == res.1) && (c.0 > res.0) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn power(&self) -> u32 {
        let mut map: HashMap<&str, u32> = HashMap::new();
        for set in &self.sets {
            for c in set {
                if (!map.contains_key(&c.1 as &str)) || (map.get(&c.1 as &str).unwrap() < &c.0) {
                    map.insert(&c.1, c.0);
                }
            }
        }

        let mut pow = 1;
        for v in map {
            pow *= v.1;
        }
        pow
    }
}

fn parse_sets(line: &str) -> Vec<Set> {
    line.split("; ")
        .map(|v| {
            v.split(", ")
                .map(|x| {
                    let mut s = x.split(' ');
                    let qty: u32 = s.next().unwrap().parse().unwrap();
                    let color = s.next().unwrap();
                    (qty, String::from(color))
                })
                .collect()
        })
        .collect()
}

fn parse_game(line: &str) -> Game {
    let id_end = line.find(':').unwrap();
    let game_id: u32 = line[5..id_end].parse().unwrap();

    let sets = parse_sets(&line[id_end + 2..]);

    let game = Game {
        id: game_id,
        sets: sets,
    };
    game
}

fn part_one(input: &str) -> Option<u32> {
    let game_list = aoc::utils::read_list_parse(input, parse_game);

    let total: Set = vec![
        (12, String::from("red")),
        (13, String::from("green")),
        (14, String::from("blue")),
    ];

    let mut sum = 0;
    for g in game_list {
        if g.valid(&total) {
            sum += g.id;
        }
    }
    Some(sum)
}

fn part_two(input: &str) -> Option<u32> {
    let game_list = aoc::utils::read_list_parse(input, parse_game);

    let sum = game_list.iter().map(|g| g.power()).sum();
    Some(sum)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 02);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 02);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 02);
        assert_eq!(part_two(&input), Some(2286));
    }
}
