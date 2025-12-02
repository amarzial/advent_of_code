use itertools::Itertools;

fn is_valid_id_p1(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return true;
    }
    let first_half = &id[..id.len() / 2];
    let second_half = &id[id.len() / 2..];
    first_half != second_half
}

fn count_invalid_ids_p1(range: &str) -> usize {
    let mut count = 0;
    let split = range.split('-').collect::<Vec<&str>>();
    let start: u64 = split[0].parse().unwrap();
    let end: u64 = split[1].parse().unwrap();
    for id in start..=end {
        let id_str = id.to_string();
        if !is_valid_id_p1(&id_str) {
            count += id as usize;
        }
    }
    count
}

fn part_one(input: &str) -> Option<String> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut total_invalid = 0;
    for range in ranges {
        total_invalid += count_invalid_ids_p1(range);
    }
    Some(total_invalid.to_string())
}

fn is_valid_id_p2(id: &str) -> bool {
    for splits in 2..=id.len() {
        if id.len() % splits != 0 {
            continue;
        }

        let mut equal = true;
        for i in 0..splits - 1 {
            let offset = i * (id.len() / splits);
            let start1 = offset;
            let end1 = start1 + id.len() / splits;
            let start2 = end1;
            let end2 = start2 + id.len() / splits;
            equal = equal && id[start1..end1] == id[start2..end2];
        }
        if equal {
            return false;
        }
    }
    return true;
}

fn count_invalid_ids_p2(range: &str) -> usize {
    let mut count = 0;
    let split = range.split('-').collect::<Vec<&str>>();
    let start: u64 = split[0].parse().unwrap();
    let end: u64 = split[1].parse().unwrap();
    for id in start..=end {
        let id_str = id.to_string();
        if !is_valid_id_p2(&id_str) {
            count += id as usize;
        }
    }
    count
}

fn part_two(input: &str) -> Option<String> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut total_invalid = 0;
    for range in ranges {
        total_invalid += count_invalid_ids_p2(range);
    }
    Some(total_invalid.to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2025, 02);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2025, 02);
        assert_eq!(part_one(&input), Some("1227775554".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2025, 02);
        assert_eq!(part_two(&input), Some("4174379265".to_string()));
    }
}
