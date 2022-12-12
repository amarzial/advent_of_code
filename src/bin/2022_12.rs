use aoc::helpers::Coordinate;

fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = aoc::utils::read_list_parse(input, |l| l.chars().collect());

    let mut start = Coordinate::new(0 as i32, 0 as i32);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'E' {
                start = Coordinate::new(x as i32, y as i32);
                break;
            }
        }
    }

    let res = pathfinding::directed::bfs::bfs(
        &start,
        |s| {
            s.neighbors()
                .into_iter()
                .filter(|n| {
                    if n.x < 0 || n.x >= grid[0].len() as i32 || n.y < 0 || n.y >= grid.len() as i32
                    {
                        return false;
                    }

                    let mut from = grid[s.y as usize][s.x as usize];
                    let mut to = grid[n.y as usize][n.x as usize];
                    if to == 'S' {
                        to = 'a';
                    };
                    if from == 'E' {
                        from = 'z';
                    };
                    (from <= to) || (from as u8 - to as u8 == 1)
                })
                .collect::<Vec<Coordinate<i32>>>()
        },
        |n| grid[n.y as usize][n.x as usize] == 'S',
    );
    Some(res.unwrap().len())
}

fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = aoc::utils::read_list_parse(input, |l| l.chars().collect());

    let mut start = Coordinate::new(0 as i32, 0 as i32);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'E' {
                start = Coordinate::new(x as i32, y as i32);
                break;
            }
        }
    }

    let res = pathfinding::directed::bfs::bfs(
        &start,
        |s| {
            s.neighbors()
                .into_iter()
                .filter(|n| {
                    if n.x < 0 || n.x >= grid[0].len() as i32 || n.y < 0 || n.y >= grid.len() as i32
                    {
                        return false;
                    }

                    let mut from = grid[s.y as usize][s.x as usize];
                    let mut to = grid[n.y as usize][n.x as usize];
                    if to == 'S' {
                        to = 'a';
                    };
                    if from == 'E' {
                        from = 'z';
                    };
                    (from <= to) || (from as u8 - to as u8 == 1)
                })
                .collect::<Vec<Coordinate<i32>>>()
        },
        |n| grid[n.y as usize][n.x as usize] == 'a' || grid[n.y as usize][n.x as usize] == 'S',
    );
    Some(res.unwrap().len())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 12);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
