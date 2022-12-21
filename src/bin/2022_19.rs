use std::vec;

type Resources = [i32; 4];

fn parse_bot(desc: &str) -> Resources {
    let mut vals = desc.split_whitespace();
    let mut bot = [0, 0, 0, 0];
    while let Some(v) = vals.next() {
        if v == "and" {
            continue;
        }
        let qty: i32 = v.parse().unwrap();
        let t = vals.next().unwrap();
        match t {
            "ore" => {
                bot[0] += qty;
            }
            "clay" => {
                bot[1] += qty;
            }
            "obsidian" => {
                bot[2] += qty;
            }
            "geode" => {
                bot[3] += qty;
            }
            _ => {}
        }
    }
    bot
}

fn parse_input(line: &str) -> [Resources; 4] {
    let fields = aoc::utils::read_pattern("Blueprint {}: Each ore robot costs {}. Each clay robot costs {}. Each obsidian robot costs {}. Each geode robot costs {}.", line).unwrap();
    let _num: i32 = fields[0].parse().unwrap();

    [
        parse_bot(fields[1]),
        parse_bot(fields[2]),
        parse_bot(fields[3]),
        parse_bot(fields[4]),
    ]
}

type State = (Resources, Resources, i32);

fn can_build(state: &State, blueprint: [Resources; 4], bot_n: usize) -> Option<State> {
    let bot = blueprint[bot_n];
    let mut out = *state;
    let mut time = 1;
    for i in 0..4 {
        if state.1[i] == 0 {
            continue;
        }
        if state.0[i] > bot[i] {
            continue;
        }
        let missing = bot[i] - state.0[i];

        let time_required = missing / state.1[i] + missing % state.1[i];

        if time_required > state.2 {
            return None;
        }
        time = time.max(time_required);
    }
    for i in 0..4 {
        out.0[i] = state.1[i] * time - bot[i];
    }
    out.1[bot_n] += 1;
    out.2 -= time;

    Some(out)
}

fn part_one(input: &str) -> Option<String> {
    let res: Resources = [0, 0, 0, 0];
    let robots: Resources = [1, 0, 0, 0];
    let blueprints = aoc::utils::read_list_parse(input, parse_input);

    let mut num = 1;
    for b in blueprints {
        let p = pathfinding::directed::dfs::dfs_reach((res, robots, 24), |state| -> Vec<State> {
            let mut possible = vec![];
            if state.2 <= 0 {
                return possible;
            }
            for i in 0..4 {
                match can_build(state, b, i) {
                    Some(s) => {
                        possible.push(s);
                    }
                    None => {}
                };
            }
            possible
        });
        let best = p.map(|s| s.1[3]).max().unwrap();
        println!("blueprint {} => {}", num, best);
        num += 1;
    }
    None
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 19);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 19);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 19);
        assert_eq!(part_two(&input), None);
    }
}
