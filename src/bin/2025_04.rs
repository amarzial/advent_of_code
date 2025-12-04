use std::collections::HashSet;

use aoc::helpers::coordinate::Coordinate;

fn create_grid(input: &str) -> HashSet<(i32, i32)> {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    grid.insert((col as i32, row as i32));
                }
                _ => {}
            }
        }
    }
    grid
}

fn part_one(input: &str) -> Option<String> {
    let grid = create_grid(input);

    let count = grid
        .iter()
        .filter(|(x, y)| {
            let c = Coordinate::new(*x, *y);
            c.neighbors_extended()
                .iter()
                .filter(|n| grid.contains(&(n.x, n.y)))
                .count()
                < 4
        })
        .count();
    Some(count.to_string())
}

fn part_two(input: &str) -> Option<String> {
    let mut grid = create_grid(input);

    let mut count = 0;

    loop {
        let removable: Vec<(i32, i32)> = grid.iter().filter(|(x, y)| {
            let c = Coordinate::new(*x, *y);
            c.neighbors_extended()
                .iter()
                .filter(|n| grid.contains(&(n.x, n.y)))
                .count()
                < 4
        }).cloned().collect();

        if removable.is_empty() {
            break;
        }
        for r in removable {
            grid.remove(&r);
            count += 1;
        }
    }

    Some(count.to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2025, 04);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2025, 04);
        assert_eq!(part_one(&input), Some("13".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2025, 04);
        assert_eq!(part_two(&input), Some("43".to_string()));
    }
}
