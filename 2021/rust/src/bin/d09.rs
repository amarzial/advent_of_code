use std::collections::{HashMap, VecDeque};

use aoc::utils;

type Grid = Vec<Vec<u32>>;

fn part_one(g: &Grid) -> Vec<usize> {
    let mut targets = Vec::new();
    let positions = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    for y in 0..g.len() {
        let row = &g[y];
        for x in 0..row.len() {
            let mut lowest = 9;
            for p in positions.iter() {
                match g.get((y as i32 + p.0) as usize) {
                    Some(v) => match v.get((x as i32 + p.1) as usize) {
                        Some(n) => {
                            lowest = lowest.min(*n);
                        }
                        None => {}
                    },
                    None => {}
                }
            }
            let current = g[y][x];

            if current < lowest {
                targets.push(y * g[0].len() + x);
            }
        }
    }
    return targets;
}

fn bfs(grid: &Grid, elem: usize, visited: &mut HashMap<usize, bool>) -> u32 {
    let mut queue = VecDeque::new();
    let height = grid.len();
    let width = grid[0].len();
    let positions: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    queue.push_back(elem);

    let mut size = 0;
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        if visited.contains_key(&current) {
            continue;
        }
        visited.insert(current, true);
        let x = (current % width) as i32;
        let y = (current / width) as i32;

        if grid[y as usize][x as usize] < 9 {
            size += 1;
            for p in positions.iter() {
                if (x + p.0 >= 0)
                    && (x + p.0 < width as i32)
                    && (y + p.1 >= 0)
                    && (y + p.1 < height as i32)
                {
                    queue.push_back((current as i32 + p.1 * width as i32 + p.0) as usize);
                }
            }
        }
    }
    return size;
}

fn part_two(g: &Grid, targets: &Vec<usize>) -> u32 {
    let mut visited: HashMap<usize, bool> = HashMap::new();
    let mut baisins = Vec::new();
    for t in targets.iter() {
        let sum = bfs(g, *t, &mut visited);
        baisins.push(sum);
    }
    baisins.sort();
    let mut it = baisins.iter().rev();

    let mut res = *it.next().unwrap();
    res *= *it.next().unwrap();
    res *= *it.next().unwrap();
    return res;
}

fn main() {
    let input: Grid = utils::read_list_parse(&utils::get_input(), |x| {
        x.chars().map(|c| c.to_digit(10).unwrap()).collect()
    });

    let targets = part_one(&input);
    let w = input[0].len();
    let p1 = targets.iter().fold(0, |tot, curr| {
        return tot + input[curr / w][curr % w] + 1;
    });
    println!("Part 1: {}", p1);
    let p2 = part_two(&input, &targets);
    println!("Part 2: {}", p2);
}
