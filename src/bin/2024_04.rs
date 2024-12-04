use aoc::helpers::coordinate::Coordinate;
type Coord = Coordinate<i32>;
type Grid = Vec<Vec<char>>;

fn build_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_word(word: &str, coord: Coord, direction: Option<Coord>, grid: &Grid) -> usize {
    if word.is_empty() {
        return 1;
    }
    if coord.x < 0
        || coord.y < 0
        || coord.y as usize >= grid.len()
        || coord.x as usize >= grid[coord.y as usize].len()
    {
        return 0;
    }
    if grid[coord.y as usize][coord.x as usize] != word.chars().next().unwrap() {
        return 0;
    }
    let mut total = 0;
    if direction.is_none() {
        let cursor = Coord::new(0, 0);
        for n in cursor.neighbors_extended() {
            total += find_word(&word[1..], coord + n, Some(n), grid);
        }
    } else {
        let new_coord = coord + direction.unwrap();
        total += find_word(&word[1..], new_coord, direction, grid);
    }
    total
}

fn count_words(word: &str, grid: &Grid) -> usize {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let coord = Coord::new(x as i32, y as i32);
            total += find_word(word, coord, None, grid);
        }
    }
    total
}

fn find_xmas(coord: Coord, grid: &Grid) -> usize {
    if grid[coord.y as usize][coord.x as usize] != 'A' {
        return 0;
    }
    if coord.x == 0
        || coord.y == 0
        || coord.x as usize >= grid[0].len() - 1
        || coord.y as usize >= grid.len() - 1
    {
        return 0;
    }
    let coords = vec![
        Coord::new(-1, -1),
        Coord::new(-1, 1),
        Coord::new(1, 1),
        Coord::new(1, -1),
    ];
    let mut ms = 0;
    let mut ss = 0;
    for n in coords {
        let ch = grid[(coord.y + n.y) as usize][(coord.x + n.x) as usize];
        match ch {
            'M' => ms += 1,
            'S' => ss += 1,
            _ => {}
        };
    }
    if ms != 2 || ss != 2 {
        return 0;
    }

    if grid[(coord.y + 1) as usize][(coord.x + 1) as usize]
        == grid[(coord.y - 1) as usize][(coord.x - 1) as usize]
    {
        return 0;
    }

    return 1;
}

fn count_part_2(grid: &Grid) -> usize {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let coord = Coord::new(x as i32, y as i32);
            total += find_xmas(coord, grid);
        }
    }
    total
}

fn part_one(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let res = count_words("XMAS", &grid);
    Some(res)
}

fn part_two(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let res = count_part_2(&grid);
    Some(res)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2024, 04);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2024, 04);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2024, 04);
        assert_eq!(part_two(&input), Some(9));
    }
}
