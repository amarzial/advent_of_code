use num::Integer;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    left: usize,
    right: usize,
}

fn index(value: &str) -> usize {
    let mut res = 0;
    for c in value.chars() {
        res *= 26;
        res += c as u32 - 'A' as u32
    }
    res as usize
}

fn parse_input(input: &str) -> (String, Vec<Node>) {
    let mut lines = aoc::utils::read_list::<String>(input).into_iter();
    let directions = lines.next().unwrap();
    lines.next();
    let mut nodes: Vec<Node> = vec![];
    nodes.resize(
        26 * 26 * 26,
        Node {
            value: String::new(),
            left: 0,
            right: 0,
        },
    );
    for n in lines {
        let elems = aoc::utils::read_pattern("{} = ({}, {})", &n).unwrap();
        let idx = index(elems[0]);
        nodes[idx] = Node {
            value: String::from(elems[0]),
            left: index(elems[1]),
            right: index(elems[2]),
        };
    }
    (directions, nodes)
}

fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse_input(input);

    let mut dirs = directions.chars().cycle();
    let mut current = 0;
    let mut steps = 0;
    let target = index("ZZZ");
    loop {
        let dir = dirs.next().unwrap();
        let node = nodes.get(current).unwrap();
        current = match dir {
            'L' => node.left,
            'R' => node.right,
            _ => panic!(),
        };
        steps += 1;
        if current == target {
            break;
        }
    }
    Some(steps)
}

fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = parse_input(input);

    let mut dirs = directions.chars().cycle();

    let mut positions: Vec<usize> = nodes
        .iter()
        .filter(|n| n.value != "" && n.value.as_bytes()[2] == b'A')
        .map(|n| index(&n.value))
        .collect();

    let mut loops = vec![];
    loops.resize(positions.len(), (0, 0, false));

    let mut steps = 0;
    loop {
        let dir = dirs.next().unwrap();
        for i in 0..positions.len() {
            let pos = positions[i];
            let node = &nodes[pos];
            positions[i] = match dir {
                'L' => node.left,
                'R' => node.right,
                _ => panic!(),
            };
        }
        steps += 1;
        let mut done = 0;
        for i in 0..positions.len() {
            let pos = positions[i];
            let node = &nodes[pos];
            if node.value.as_bytes()[2] == b'Z' {
                let loop_size = steps - loops[i].0;
                let full = loop_size == loops[i].1;
                if !loops[i].2 {
                    loops[i] = (steps, loop_size, full);
                }
            }
            if loops[i].2 {
                done += 1;
            }
        }
        if done == positions.len() {
            break;
        }
    }
    println!("{:?}", loops);
    let mut lcm = 1;
    for l in loops {
        lcm = lcm.lcm(&(l.1 as u64));
    }

    Some(lcm)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 08);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 08);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 08);
        assert_eq!(part_two(&input), Some(6));
    }
}
