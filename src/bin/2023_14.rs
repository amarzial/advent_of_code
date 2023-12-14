use aoc::helpers::coordinate::Coordinate;
use itertools::{Either, Itertools};
use pathfinding::cycle_detection::floyd;

type Grid = Vec<Vec<char>>;

fn move_grid(grid: &mut Grid, offset: Coordinate<i32>) {
    let cols = grid[0].len();
    let rows = grid.len();
    let reverse = offset.x > 0 || offset.y > 0;
    for y in if !reverse {
        Either::Left(0..rows)
    } else {
        Either::Right((0..rows).rev())
    } {
        for x in if !reverse {
            Either::Left(0..cols)
        } else {
            Either::Right((0..cols).rev())
        } {
            let c = grid[y][x];
            match c {
                'O' => {
                    let mut cursor = Coordinate::new(x as i32, y as i32);
                    let start = cursor;

                    loop {
                        if (cursor.x == 0 && offset.x < 0)
                            || (cursor.x == cols as i32 - 1 && offset.x > 0)
                            || (cursor.y == 0 && offset.y < 0)
                            || (cursor.y == rows as i32 - 1 && offset.y > 0)
                        {
                            break;
                        }

                        let new_y = (cursor.y as i32 + offset.y) as usize;
                        let new_x = (cursor.x as i32 + offset.x) as usize;
                        if grid[new_y][new_x] != '.' {
                            break;
                        }
                        cursor += offset;
                    }
                    if cursor != start {
                        let tmp = grid[start.y as usize][start.x as usize];
                        grid[start.y as usize][start.x as usize] =
                            grid[cursor.y as usize][cursor.x as usize];
                        grid[cursor.y as usize][cursor.x as usize] = tmp;
                    }
                }
                _ => {}
            };
        }
    }
}

fn calc_load(grid: &Grid) -> usize {
    let mut load = 0;
    let height = grid.len();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            load += match grid[y][x] {
                'O' => height - y,
                _ => 0,
            };
        }
    }
    load
}

fn part_one(input: &str) -> Option<usize> {
    let mut grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();
    move_grid(&mut grid, Coordinate::new(0, -1));
    Some(calc_load(&grid))
}

fn cycle(grid: &mut Grid) {
    move_grid(grid, Coordinate::new(0, -1));
    move_grid(grid, Coordinate::new(-1, 0));
    move_grid(grid, Coordinate::new(0, 1));
    move_grid(grid, Coordinate::new(1, 0));
}

fn part_two(input: &str) -> Option<usize> {
    let mut grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let (loop_size, state, loop_start) = floyd(grid, |mut g| {
        cycle(&mut g);
        g
    });
    // println!("cycle found at: {}, length: {}", loop_start, loop_size);
    let full_cycles = 1000000000;
    let remaining = (full_cycles - loop_start) % loop_size + loop_size;
    grid = state;
    for _ in 0..remaining {
        cycle(&mut grid);
    }

    Some(calc_load(&grid))
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 14);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 14);
        assert_eq!(part_one(&input), Some(136));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 14);
        assert_eq!(part_two(&input), Some(64));
    }
}
