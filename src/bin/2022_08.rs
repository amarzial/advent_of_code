use std::collections::HashSet;

use num::range_step_inclusive;

enum Direction {
    TopBottom,
    BottomTop,
    LeftRight,
    RightLeft,
}

type Grid = Vec<Vec<u8>>;

fn fetch_side(
    grid: &Grid,
    dir: Direction,
    visible_trees: &mut HashSet<usize>,
    scores: &mut Vec<usize>,
) {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let (range1, range2, row_first) = match dir {
        Direction::TopBottom => (0..width, range_step_inclusive(0, height - 1, 1), true),
        Direction::BottomTop => (0..width, range_step_inclusive(height - 1, 0, -1), true),
        Direction::LeftRight => (0..height, range_step_inclusive(0, width - 1, 1), false),
        Direction::RightLeft => (0..height, range_step_inclusive(width - 1, 0, -1), false),
    };

    for i in range1 {
        let mut previous = 0;
        let mut points = 1;
        for j in range2.clone() {
            let (x, y) = if row_first {
                (j as usize, i as usize)
            } else {
                (i as usize, j as usize)
            };
            let c = grid[x][y];
            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                visible_trees.insert(x * width as usize + y);
            } else {
                if c > previous {
                    visible_trees.insert(x * width as usize + y);
                }
            }
            if c <= previous {
                points = 1;
            }
            scores[x * width as usize + y] *= if points > 0 { points } else { 1 };
            points += 1;
            previous = previous.max(c);
        }
    }
}

fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<u8>> = aoc::utils::read_list_parse(input, |l| {
        l.as_bytes().iter().map(|c| c - '0' as u8).collect()
    });
    let mut visible_trees = HashSet::new();
    let mut points: Vec<usize> = Vec::new();
    points.resize(grid.len() * grid[0].len(), 1);

    fetch_side(&grid, Direction::TopBottom, &mut visible_trees, &mut points);
    fetch_side(&grid, Direction::BottomTop, &mut visible_trees, &mut points);
    fetch_side(&grid, Direction::LeftRight, &mut visible_trees, &mut points);
    fetch_side(&grid, Direction::RightLeft, &mut visible_trees, &mut points);

    Some(visible_trees.len())
}

fn view_score(grid: &Grid, x: usize, y: usize) -> usize {
    let tree = grid[y][x];
    let mut distance = 0;
    let mut score = 1;
    for row in (0..y).rev() {
        distance += 1;
        if grid[row][x] >= tree {
            break;
        }
    }
    score *= distance;
    distance = 0;
    for row in y + 1..grid.len() {
        distance += 1;
        if grid[row][x] >= tree {
            break;
        }
    }
    score *= distance;
    distance = 0;
    for col in (0..x).rev() {
        distance += 1;
        if grid[y][col] >= tree {
            break;
        }
    }
    score *= distance;
    distance = 0;
    for col in x + 1..grid[0].len() {
        distance += 1;
        if grid[y][col] >= tree {
            break;
        }
    }
    score *= distance;

    score
}

fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<u8>> = aoc::utils::read_list_parse(input, |l| {
        l.as_bytes().iter().map(|c| c - '0' as u8).collect()
    });

    let mut best = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            best = best.max(view_score(&grid, x, y));
        }
    }
    Some(best)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 08);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 08);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 08);
        assert_eq!(part_two(&input), Some(8));
    }
}
