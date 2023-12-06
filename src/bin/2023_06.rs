type Int = i64;

struct Race {
    time: Int,
    distance: Int,
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines = aoc::utils::read_list::<String>(input);

    let times: Vec<Int> = lines[0]
        .chars()
        .skip(5)
        .collect::<String>()
        .split(' ')
        .filter(|s| s != &"")
        .map(|v| v.parse::<Int>().unwrap())
        .collect();

    let distances: Vec<Int> = lines[1]
        .chars()
        .skip(9)
        .collect::<String>()
        .split(' ')
        .filter(|s| s != &"")
        .map(|v| v.parse::<Int>().unwrap())
        .collect();

    let mut races = vec![];
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    races
}

impl Race {
    fn find_load_times(self) -> (Int, Int) {
        let a = -1.0;
        let b = self.time as f64;
        let c = -self.distance as f64;

        let delta = f64::sqrt(b.powf(2.) - 4. * (a * c));

        let result1 = (-b + delta) / (2. * a);
        let result2 = (-b - delta) / (2. * a);

        (result1 as Int + 1, (result2.ceil() - 1.) as Int)
    }
}

fn part_one(input: &str) -> Option<Int> {
    let races = parse_races(input);

    let mut tot = 1;
    for r in races {
        let res = r.find_load_times();
        let ways = 1 + res.1 - res.0;
        tot *= ways;
    }
    Some(tot)
}

fn part_two(input: &str) -> Option<Int> {
    let races = parse_races(input);
    let time = races
        .iter()
        .map(|r| r.time.to_string())
        .fold(String::new(), |tot, curr| tot + &curr)
        .parse::<Int>()
        .unwrap();

    let distance = races
        .iter()
        .map(|r| r.distance.to_string())
        .fold(String::new(), |tot, curr| tot + &curr)
        .parse::<Int>()
        .unwrap();

    let res = Race { time, distance }.find_load_times();
    Some(res.1 - res.0 + 1)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 06);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 06);
        assert_eq!(part_one(&input), Some(288));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 06);
        assert_eq!(part_two(&input), Some(71503));
    }
}
