use aoc::helpers::coordinate::Coordinate;
use itertools::Itertools;
type Coord = Coordinate<i32>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    coord: Coord,
    direction: Coord,
    direction_steps: u32,
}

fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let end = Coord::new(grid[0].len() as i32 - 1, grid.len() as i32 - 1);

    let path = pathfinding::prelude::dijkstra(
        &Node {
            coord: Coord::new(0, 0),
            direction: Coord::new(0, 0),
            direction_steps: 0,
        },
        |node| {
            node.coord
                .neighbors()
                .into_iter()
                .filter(|ne| {
                    ne.x >= 0
                        && ne.y >= 0
                        && ne.x < grid[0].len() as i32
                        && ne.y < grid.len() as i32
                })
                .map(|ne| {
                    let dir = ne - node.coord;
                    let dir_steps = if node.direction == dir {
                        node.direction_steps + 1
                    } else {
                        1
                    };
                    let heat = grid[ne.y as usize][ne.x as usize];
                    (
                        Node {
                            coord: ne,
                            direction: dir,
                            direction_steps: dir_steps,
                        },
                        heat,
                    )
                })
                .filter(|nx| {
                    let prev = node.coord - node.direction;
                    !(nx.0.direction_steps >= 4) && nx.0.coord != prev
                })
                .collect_vec()
        },
        |c| c.coord == end,
    )
    .unwrap();

    Some(path.1)
}

fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let end = Coord::new(grid[0].len() as i32 - 1, grid.len() as i32 - 1);

    let path = pathfinding::prelude::dijkstra(
        &Node {
            coord: Coord::new(0, 0),
            direction: Coord::new(0, 0),
            direction_steps: 0,
        },
        |node| {
            node.coord
                .neighbors()
                .into_iter()
                .filter(|ne| {
                    ne.x >= 0
                        && ne.y >= 0
                        && ne.x < grid[0].len() as i32
                        && ne.y < grid.len() as i32
                })
                .map(|ne| {
                    let dir = ne - node.coord;
                    let dir_steps = if node.direction == dir {
                        node.direction_steps + 1
                    } else {
                        1
                    };
                    let heat = grid[ne.y as usize][ne.x as usize];
                    (
                        Node {
                            coord: ne,
                            direction: dir,
                            direction_steps: dir_steps,
                        },
                        heat,
                    )
                })
                .filter(|nx| {
                    if node.direction != Coord::new(0, 0)
                        && (nx.0.direction != node.direction)
                        && node.direction_steps < 4
                    {
                        return false;
                    }
                    let prev = node.coord - node.direction;
                    nx.0.direction_steps < 11 && nx.0.coord != prev
                })
                .collect_vec()
        },
        |c| c.coord == end && c.direction_steps >= 4,
    )
    .unwrap();

    Some(path.1)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 17);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 17);
        assert_eq!(part_one(&input), Some(102));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 17);
        assert_eq!(part_two(&input), Some(94));
    }
}
