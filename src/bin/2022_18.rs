use std::collections::HashSet;

use aoc::helpers::coordinate3d::Coordinate3D;

type Coord = Coordinate3D<i32>;

fn build_input(input: &str) -> HashSet<Coord> {
    let mut grid: HashSet<Coord> = HashSet::new();

    for mut c in input.lines().filter(|s| s.len() > 0).map(|l| l.split(',')) {
        let coord = Coord::new(
            c.next().unwrap().parse().unwrap(),
            c.next().unwrap().parse().unwrap(),
            c.next().unwrap().parse().unwrap(),
        );

        grid.insert(coord);
    }
    grid
}

fn part_one(input: &str) -> Option<i32> {
    let grid = build_input(input);

    let tot_surface = grid.iter().fold(0, |partial_sum, c| {
        partial_sum
            + (6 - c
                .neighbors()
                .iter()
                .fold(0, |tot, p| tot + if grid.contains(p) { 1 } else { 0 }))
    });

    Some(tot_surface)
}

fn part_two(input: &str) -> Option<i32> {
    let grid = build_input(input);

    let min = Coord::new(0, 0, 0);
    let mut max = grid.iter().fold(Coord::new(1, 1, 1), |max, next| {
        Coord::new(max.x.max(next.x), max.y.max(next.y), max.z.max(next.z))
    });

    max += Coord::new(1, 1, 1);

    let filled = pathfinding::directed::bfs::bfs_reach(Coord::new(0, 0, 0), |c| {
        c.neighbors()
            .iter()
            .cloned()
            .filter(|c| {
                if grid.contains(c) {
                    return false;
                }
                (c.x >= min.x)
                    && (c.y >= min.y)
                    && (c.z >= min.z)
                    && (c.x <= max.x)
                    && (c.y <= max.y)
                    && (c.z <= max.z)
            })
            .collect::<Vec<Coord>>()
    });

    let tot_surface = filled.fold(0, |partial_sum, c| {
        let sum = c
            .neighbors()
            .iter()
            .fold(0, |tot, p| tot + if grid.contains(p) { 1 } else { 0 });
        partial_sum + sum
    });

    Some(tot_surface)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 18);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
