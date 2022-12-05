#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<char>>;
type Moves = Vec<Move>;

fn parse_input(input: &str) -> (Stacks, Moves) {
    let lines = aoc::utils::read_list::<String>(input);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    for l in lines {
        if l.starts_with('m') {
            let values: Vec<usize> = aoc::utils::read_pattern("move {} from {} to {}", &l)
                .unwrap()
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();
            let count = values[0];
            let from = values[1];
            let to = values[2];
            moves.push(Move { count, from, to });
        } else {
            let mut cnt = 0;
            for i in (0..l.len()).step_by(4) {
                if i + 3 > l.len() {
                    continue;
                }
                if stacks.len() <= cnt {
                    stacks.push(Vec::new());
                }
                let piece = &l[i..i + 3];
                let mut c = piece.chars();
                if c.nth(0).unwrap() == '[' {
                    let letter = c.next().unwrap();
                    {
                        stacks.get_mut(cnt).unwrap().insert(0, letter)
                    }
                }
                cnt += 1;
            }
        }
    }
    (stacks, moves)
}

fn part_one(input: &(Stacks, Moves)) -> Option<String> {
    let mut stack = input.0.clone();
    let moves = &input.1;

    for m in moves {
        for _i in 0..m.count {
            let letter = stack[m.from - 1].pop().unwrap();
            stack[m.to - 1].push(letter);
        }
    }

    let mut out = String::new();
    for s in stack {
        match s.last() {
            Some(c) => {
                out.push(*c);
            }
            None => {}
        }
    }
    Some(out)
}

fn part_two(input: &(Stacks, Moves)) -> Option<String> {
    let mut stack = input.0.clone();
    let moves = &input.1;

    for m in moves {
        let len = stack[m.from - 1].len();
        let letters = stack[m.from - 1].split_off(len - m.count);
        stack[m.to - 1].extend(letters);
    }

    let mut out = String::new();
    for s in stack {
        match s.last() {
            Some(c) => {
                out.push(*c);
            }
            None => {}
        }
    }
    Some(out)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 05);
    let parsed = parse_input(&input);

    aoc::solve!(1, part_one, &parsed);
    aoc::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 05);
        let parsed = parse_input(&input);
        assert_eq!(part_one(&parsed), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 05);
        let parsed = parse_input(&input);
        assert_eq!(part_two(&parsed), Some(String::from("MCD")));
    }
}
