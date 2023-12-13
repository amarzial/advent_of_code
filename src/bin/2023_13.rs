use itertools::Itertools;

type Int = u32;
struct Map {
    rows: Vec<Int>,
    columns: Vec<Int>,
}

impl Map {
    fn _dim_reflections(axis: &Vec<Int>, exclude: Option<usize>, smudge: bool) -> usize {
        let reflection = axis
            .iter()
            .enumerate()
            .zip(axis.iter().enumerate().skip(1))
            .find(|(a, b)| {
                let mut tolerant = smudge;
                let same = a.1 == b.1;
                if !same {
                    if tolerant {
                        let diff = a.1 ^ b.1;
                        tolerant = false;
                        if !(diff != 0 && (diff & (diff - 1) == 0)) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                let perfect = (0..a.0)
                    .rev()
                    .zip((b.0..axis.len()).skip(1))
                    .all(|(lhs, rhs)| {
                        if axis[lhs] == axis[rhs] {
                            true
                        } else {
                            match tolerant {
                                false => false,
                                true => {
                                    let diff = axis[lhs] ^ axis[rhs];
                                    tolerant = false;
                                    diff != 0 && (diff & (diff - 1) == 0)
                                }
                            }
                        }
                    });
                if perfect {
                    match exclude {
                        Some(x) => b.0 != x,
                        None => true,
                    }
                } else {
                    false
                }
            });
        match reflection {
            Some(v) => v.1 .0,
            None => 0,
        }
    }

    fn reflections(&self) -> (usize, usize) {
        (
            Self::_dim_reflections(&self.rows, None, false),
            Self::_dim_reflections(&self.columns, None, false),
        )
    }

    fn reflections2(&mut self) -> (usize, usize) {
        let r1 = Self::_dim_reflections(&self.rows, None, false);
        let r2 = Self::_dim_reflections(&self.columns, None, false);
        (
            Self::_dim_reflections(&self.rows, Some(r1), true),
            Self::_dim_reflections(&self.columns, Some(r2), true),
        )
    }
}

fn parse(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .map(|g| {
            let grid = g.lines().collect_vec();
            let rows = grid
                .iter()
                .map(|l| Int::from_str_radix(&(l.replace('#', "1")).replace('.', "0"), 2).unwrap())
                .collect_vec();
            let cols = (0..grid[0].len())
                .map(|x| {
                    let mut i: Int = 0;
                    for y in 0..grid.len() {
                        i <<= 1;
                        i |= match grid[y].as_bytes()[x] {
                            b'#' => 1,
                            _ => 0,
                        }
                    }
                    i
                })
                .collect_vec();
            Map {
                rows,
                columns: cols,
            }
        })
        .collect_vec()
}

fn part_one(input: &str) -> Option<usize> {
    let maps = parse(input);

    let sum = maps
        .iter()
        .map(|v| {
            let r = v.reflections();
            100 * r.0 + r.1
        })
        .sum();
    Some(sum)
}

fn part_two(input: &str) -> Option<usize> {
    let maps = parse(input);

    let sum = maps
        .into_iter()
        .map(|mut v| {
            let r = v.reflections2();
            100 * r.0 + r.1
        })
        .sum();
    Some(sum)
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
        assert_eq!(part_two(&input), Some(400));
    }
}
