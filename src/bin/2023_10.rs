use aoc::helpers::coordinate::Coordinate;

type Int = i32;

fn find_s(coord: Coordinate<Int>, grid: &Vec<Vec<char>>) -> char {
    let nord = coord + Coordinate::new(0, -1);
    let sud: Coordinate<i32> = coord + Coordinate::new(0, 1);
    let est: Coordinate<i32> = coord + Coordinate::new(1, 0);
    let ovest: Coordinate<i32> = coord + Coordinate::new(-1, 0);

    let mut is_nord = false;
    let mut is_sud = false;
    let mut is_est = false;
    let mut is_ovest = false;

    if nord.y >= 0 {
        let c = grid[nord.y as usize][nord.x as usize];
        is_nord = match c {
            '|' | 'F' | '7' => true,
            _ => false,
        }
    }
    if sud.y < grid.len() as i32 {
        let c = grid[sud.y as usize][sud.x as usize];
        is_sud = match c {
            '|' | 'L' | 'J' => true,
            _ => false,
        }
    }
    if est.x < grid[0].len() as i32 {
        let c = grid[est.y as usize][est.x as usize];
        is_est = match c {
            '-' | '7' | 'J' => true,
            _ => false,
        }
    }
    if ovest.x >= 0 {
        let c = grid[ovest.y as usize][ovest.x as usize];
        is_ovest = match c {
            '-' | 'L' | 'F' => true,
            _ => false,
        }
    }

    match (is_nord, is_sud, is_est, is_ovest) {
        (true, true, false, false) => '|',
        (true, false, true, false) => 'L',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '7',
        (false, false, true, true) => '-',
        _ => ' ',
    }
}

fn neighbors(coord: Coordinate<Int>, c: char) -> (Coordinate<Int>, Coordinate<Int>) {
    match c {
        '|' => (
            coord + Coordinate::new(0, -1),
            coord + Coordinate::new(0, 1),
        ),
        '-' => (
            coord + Coordinate::new(-1, 0),
            coord + Coordinate::new(1, 0),
        ),
        'L' => (
            coord + Coordinate::new(0, -1),
            coord + Coordinate::new(1, 0),
        ),
        'J' => (
            coord + Coordinate::new(0, -1),
            coord + Coordinate::new(-1, 0),
        ),
        '7' => (
            coord + Coordinate::new(0, 1),
            coord + Coordinate::new(-1, 0),
        ),
        'F' => (coord + Coordinate::new(0, 1), coord + Coordinate::new(1, 0)),
        _ => panic!("Panic {}", c as char),
    }
}

fn part_one(input: &str) -> Option<Int> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = Coordinate::new(0, 0);
    let mut start_char = ' ';
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            let coord = Coordinate::new(x as i32, y as i32);
            if c == 'S' {
                start_char = find_s(coord, &grid);
                start = coord;
            }
        }
    }

    let paths = neighbors(start, start_char);
    let mut previous = start;
    let mut cursor = paths.0;
    let mut step_dist = vec![0, 1];
    let mut steps = 1;
    loop {
        let c = grid[cursor.y as usize][cursor.x as usize];
        if c == 'S' {
            break;
        } else if c == ' ' {
            continue;
        }
        let next = neighbors(cursor, c);

        let new = if next.0 == previous { next.1 } else { next.0 };
        previous = cursor;
        cursor = new;

        steps += 1;
        step_dist.push(steps);
    }

    Some(steps / 2 + steps % 2)
}

fn inflate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = vec![];
    out.resize(
        grid.len() * 3,
        (0..(grid[0].len() * 3)).map(|_| ' ').collect(),
    );

    for row in grid.iter().enumerate() {
        for column in row.1.iter().enumerate() {
            let pattern = match column.1 {
                '|' => [' ', 'x', ' ', ' ', '|', ' ', ' ', 'x', ' '],
                'L' => [' ', 'x', ' ', ' ', 'L', 'x', ' ', ' ', ' '],
                'J' => [' ', 'x', ' ', 'x', 'J', ' ', ' ', ' ', ' '],
                '7' => [' ', ' ', ' ', 'x', '7', ' ', ' ', 'x', ' '],
                'F' => [' ', ' ', ' ', ' ', 'F', 'x', ' ', 'x', ' '],
                '-' => [' ', ' ', ' ', 'x', '-', 'x', ' ', ' ', ' '],
                _ => [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            };
            let y = row.0 * 3;
            let x = column.0 * 3;

            out[y][x] = pattern[0];
            out[y][x + 1] = pattern[1];
            out[y][x + 2] = pattern[2];
            out[y + 1][x] = pattern[3];
            out[y + 1][x + 1] = pattern[4];
            out[y + 1][x + 2] = pattern[5];
            out[y + 2][x] = pattern[6];
            out[y + 2][x + 1] = pattern[7];
            out[y + 2][x + 2] = pattern[8];
        }
    }
    out
}

fn deflate(inflated: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = vec![];
    out.resize(
        inflated.len() / 3,
        (0..(inflated[0].len() / 3)).map(|_| ' ').collect(),
    );

    for y in 0..(inflated.len() / 3) {
        for x in 0..(inflated[0].len() / 3) {
            let center = Coordinate::new(x * 3 + 1, y * 3 + 1);
            let contained = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                .map(|c| Coordinate::new(center.x as Int + c.0, center.y as Int + c.1))
                .iter()
                .all(|c| inflated[c.y as usize][c.x as usize] == '#');
            out[y][x] = if contained {
                '#'
            } else {
                inflated[center.y][center.x]
            };
        }
    }
    out
}

fn part_two(input: &str) -> Option<Int> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = Coordinate::new(0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            let coord = Coordinate::new(x as i32, y as i32);
            if c == 'S' {
                grid[y][x] = find_s(coord, &grid);
                start = coord;
            }
        }
    }

    let mut inflated = inflate(&grid);

    start *= Coordinate::new(3, 3);
    start += Coordinate::new(2, 2);

    let res: Vec<Coordinate<Int>> = pathfinding::prelude::bfs_reach(start, |node| {
        node.neighbors().into_iter().filter(|n| {
            n.y >= 0
                && n.y < inflated.len() as i32
                && n.x >= 0
                && n.x < inflated[0].len() as i32
                && inflated[n.y as usize][n.x as usize] == ' '
        })
    })
    .collect();

    for i in res {
        inflated[i.y as usize][i.x as usize] = '#';
    }

    let def = deflate(&inflated);

    Some(
        def.iter()
            .map(|l| l.into_iter().filter(|i| **i == '#').count() as i32)
            .sum(),
    )
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 10);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 10);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 10);
        assert_eq!(part_two(&input), Some(1));
    }
}
