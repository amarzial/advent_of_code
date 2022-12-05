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
            let mut m = l.split(' ');
            let count = m.nth(1).unwrap().parse::<usize>().expect(&l);
            let from = m.nth(1).unwrap().parse::<usize>().unwrap();
            let to = m.nth(1).unwrap().parse::<usize>().unwrap();
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

fn part_one(input: &str) -> Option<String> {
    let (mut stack, moves) = parse_input(input);

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

fn part_two(input: &str) -> Option<String> {
    let (mut stack, moves) = parse_input(input);

    for m in moves {
        let mut items = Vec::new();
        for _i in 0..m.count {
            let letter = stack[m.from - 1].pop().unwrap();
            items.insert(0, letter);
        }
        stack[m.to - 1].extend(items);
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
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 05);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 05);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
