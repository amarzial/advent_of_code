fn part_one(input: &str) -> Option<String> {
    let splitter: Vec<&str> = input.lines().collect();
    let start = splitter[0].find('S').unwrap();
    let width = splitter[0].len();
    let mut split_count = 0;
    let _ = pathfinding::prelude::bfs_reach(start, |current| {
        let next_row = current / width + 1;
        let current_col = current % width;
        let next = next_row * width + current_col;

        if next_row >= splitter.len() {
            return vec![];
        }
        match splitter[next_row].as_bytes()[current_col] as char {
            '.' => vec![next],
            '^' => {
                split_count += 1;
                vec![next - 1, next + 1]
            }
            _ => panic!(),
        }
    })
    .count();

    Some(split_count.to_string())
}

fn _show(counts: &Vec<Vec<i64>>) {
    for row in counts {
        for val in row {
            print!("{:2} ", val);
        }
        println!("");
    }
}

fn fill(counts: &mut Vec<Vec<i64>>, path: &Vec<(usize, usize)>, qty: i64) {
    if qty == 0 {
        return;
    }
    for c in path {
        counts[c.1][c.0] += qty;
    }
}

fn part_two(input: &str) -> Option<String> {
    let splitter: Vec<&str> = input.lines().collect();
    let start = splitter[0].find('S').unwrap();
    let width = splitter[0].len();

    let mut counts: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.chars().map(|_| 0).collect())
        .collect();

    let mut stack: Vec<(usize, usize)> = Vec::new();

    let _ = pathfinding::prelude::dfs_reach(start, |current| {
        let next_row = current / width + 1;
        let current_row = next_row - 1;
        let current_col = current % width;
        let next = next_row * width + current_col;

        while stack.len() >= next_row {
            stack.pop();
        }
        stack.push((current_col, current_row));

        if next_row >= splitter.len() {
            fill(&mut counts, &stack, 1);
            return vec![];
        }
        match splitter[next_row].as_bytes()[current_col] as char {
            '.' => {
                let n = counts[next_row][current_col];
                fill(&mut counts, &stack, n);
                vec![next]
            }
            '^' => {
                let l = counts[next_row][current_col - 1];
                fill(&mut counts, &stack, l);
                let r = counts[next_row][current_col + 1];
                fill(&mut counts, &stack, r);
                vec![next - 1, next + 1]
            }
            _ => panic!(),
        }
    })
    .count();

    Some(counts[stack[0].1][stack[0].0].to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2025, 07);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2025, 07);
        assert_eq!(part_one(&input), Some("21".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2025, 07);
        assert_eq!(part_two(&input), Some("40".to_string()));
    }
}
