use itertools::Itertools;

struct Map {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
}

impl Map {
    fn _dim_reflections(axis: &Vec<Vec<char>>) -> usize {
        let reflection = axis
            .iter()
            .enumerate()
            .zip(axis.iter().enumerate().skip(1))
            .find(|(a, b)| {
                let same = a.1.iter().zip(b.1.iter()).all(|(j, k)| *j == *k);
                if !same {
                    return false;
                }
                let perfect = (0..a.0)
                    .rev()
                    .zip((b.0..axis.len()).skip(1))
                    .all(|(lhs, rhs)| {
                        axis[lhs]
                            .iter()
                            .zip(axis[rhs].iter())
                            .all(|(j, k)| *j == *k)
                    });
                perfect
            });
        match reflection {
            Some(v) => v.1 .0,
            None => 0,
        }
    }

    fn reflections(&self) -> (usize, usize) {
        (
            Self::_dim_reflections(&self.rows),
            Self::_dim_reflections(&self.columns),
        )
    }
}

fn part_one(input: &str) -> Option<usize> {
    let maps = input.split("\n\n").map(|g| {
        let rows = g.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let cols = (0..rows[0].len())
            .map(|x| (0..rows.len()).map(|y| rows[y][x]).collect_vec())
            .collect_vec();
        Map {
            rows,
            columns: cols,
        }
    });

    let sum = maps
        .map(|v| {
            let r = v.reflections();
            100 * r.0 + r.1
        })
        .sum();
    Some(sum)
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 13);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 13);
        assert_eq!(part_one(&input), Some(405));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 13);
        assert_eq!(part_two(&input), None);
    }
}
