use std::collections::HashMap;

use aoc::helpers::Coordinate;

fn parse_point(line: &str) -> (i32, i32) {
    let mut coords = line.split(", ");
    let x = coords.next().unwrap().parse::<i32>().unwrap();
    let y = coords.next().unwrap().parse::<i32>().unwrap();
    (x, y)
}

fn find_best(
    points: &Vec<(i32, i32)>,
    x: i32,
    y: i32,
    score: &mut HashMap<usize, i32>,
) -> Option<usize> {
    let mut min = i32::MAX;
    for i in 0..points.len() {
        let p = points[i];
        let v = Coordinate::manhattan(&Coordinate::from(p), &Coordinate::from((x, y)));
        min = min.min(v);
        score.insert(i, v);
    }

    let mut winner: Option<usize> = None;
    for (k, v) in score.into_iter() {
        if *v == min {
            if winner.is_some() {
                return None;
            }
            winner = Some(*k);
        }
    }
    winner
}

fn part_one(input: &str) -> Option<String> {
    let points = aoc::utils::read_list_parse(input, parse_point);

    let mut count = HashMap::new();
    let mut tl = (i32::MAX, i32::MAX);
    let mut br = (i32::MIN, i32::MIN);
    for i in 0..points.len() {
        let p = points[i];
        tl.0 = i32::min(tl.0, p.0);
        tl.1 = i32::min(tl.1, p.1);
        br.0 = i32::max(br.0, p.0);
        br.1 = i32::max(br.1, p.1);
        count.insert(i, 0);
    }

    let mut region = 0;
    let mut score: HashMap<usize, i32> = count.clone();
    let mut infinite = HashMap::new();
    for x in tl.0..br.0 + 1 {
        for y in tl.1..br.1 + 1 {
            score.clear();
            let winner = find_best(&points, x, y, &mut score);
            let mut sum = 0;
            for v in score.values() {
                sum += v;
            }
            if sum < 10000 {
                region += 1;
            }
            match winner {
                Some(w) => {
                    if (x == tl.0) || (x == br.0) || (y == tl.1) || (y == br.1) {
                        infinite.insert(w, 1);
                    }
                    if !infinite.contains_key(&w) {
                        count.insert(w, count.get(&w).unwrap_or(&0) + 1);
                    } else {
                        count.insert(w, 0);
                    }
                }
                None => {}
            }
        }
    }

    aoc::utils::cache(Some(region.to_string()));
    Some(count.values().max().unwrap().to_string())
}

fn part_two(input: &str) -> Option<String> {
    match aoc::utils::cache(None) {
        Some(r) => Some(r),
        None => {
            part_one(input);
            aoc::utils::cache(None)
        }
    }
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2018, 6);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2018, 6);
        assert_eq!(part_one(&input), Some(String::from("17")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2018, 6);
        assert_eq!(part_two(&input), Some(String::from("16")));
    }
}
