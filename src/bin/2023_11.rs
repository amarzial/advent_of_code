use aoc::helpers::coordinate::Coordinate;

type Grid = Vec<Vec<char>>;
type Int = i64;

fn calc_expansion(grid: &Grid, amount: Int) -> (Vec<Int>, Vec<Int>) {
    let mut rows = vec![];
    let mut columns = vec![];
    rows.resize(grid.len(), 0);
    columns.resize(grid[0].len(), 0);

    let mut spaces = 0;
    for row in grid.iter().enumerate() {
        let gap = row.1.iter().all(|c| *c == '.');
        if gap {
            spaces += amount;
        }
        rows[row.0] = spaces;
    }

    spaces = 0;
    for x in 0..grid[0].len() {
        let mut gap = true;
        for y in 0..grid.len() {
            if grid[y][x] == '#' {
                gap = false;
                break;
            }
        }
        if gap {
            spaces += amount;
        }
        columns[x] = spaces;
    }
    return (rows, columns);
}

fn get_galaxies(grid: &Grid, expansion: &(Vec<Int>, Vec<Int>)) -> Vec<Coordinate<Int>> {
    let mut v = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '#' {
                continue;
            }
            let c = Coordinate::new(x as Int + expansion.1[x], y as Int + expansion.0[y]);
            v.push(c);
        }
    }
    v
}

fn part_one(input: &str) -> Option<Int> {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let expansion = calc_expansion(&grid, 1);
    let galaxies = get_galaxies(&grid, &expansion);

    let mut git = galaxies.iter();

    let mut distances = 0;
    loop {
        match git.next() {
            Some(c) => {
                let cur = git.clone();
                for other in cur {
                    let dist = c.manhattan(other);
                    distances += dist;
                }
            }
            None => break,
        }
    }
    Some(distances)
}

fn part_two(input: &str) -> Option<Int> {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let expansion = calc_expansion(&grid, 999999);
    let galaxies = get_galaxies(&grid, &expansion);

    let mut git = galaxies.iter();

    let mut distances = 0;
    loop {
        match git.next() {
            Some(c) => {
                let cur = git.clone();
                for other in cur {
                    let dist = c.manhattan(other);
                    distances += dist;
                }
            }
            None => break,
        }
    }
    Some(distances)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 11);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 11);
        assert_eq!(part_one(&input), Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 11);
        assert_eq!(part_two(&input), None);
    }
}
