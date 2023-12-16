type Lens = (String, u8);
type LensBox = Vec<Lens>;

fn part_one(input: &str) -> Option<i32> {
    let res: i32 = input
        .lines()
        .next()
        .map(|l| {
            l.split(',')
                .map(|seq| {
                    seq.as_bytes()
                        .iter()
                        .fold(0, |tot, b| ((tot + *b as i32) * 17) % 256)
                })
                .sum::<i32>() as i32
        })
        .unwrap();
    Some(res)
}

fn part_two(input: &str) -> Option<usize> {
    let line = input.lines().next().unwrap();
    let mut boxes: Vec<LensBox> = vec![];
    boxes.resize(256, vec![]);

    line.split(',').for_each(|label| {
        let letters_size = label.find(|c: char| !c.is_ascii_alphabetic()).unwrap();
        let letters = std::str::from_utf8(&label.as_bytes()[0..letters_size]).unwrap();
        let symbol = label.as_bytes()[letters_size] as char;
        let lens_num = (label.as_bytes().iter().nth(letters_size + 1))
            .map(|u| (*u as char).to_digit(10).unwrap() as u8);

        let box_num = letters
            .as_bytes()
            .iter()
            .fold(0, |tot, c| ((tot + *c as i32) * 17) % 256) as usize;

        let lens_idx = boxes[box_num]
            .iter()
            .enumerate()
            .find(|(_, v)| &v.0 == letters)
            .map(|v| v.0);
        match symbol {
            '-' => match lens_idx {
                Some(i) => {
                    boxes[box_num].remove(i);
                }
                None => {}
            },
            '=' => {
                let new_lens = (String::from(letters), lens_num.unwrap() as u8);
                match lens_idx {
                    Some(i) => {
                        boxes[box_num][i] = new_lens;
                    }
                    None => {
                        boxes[box_num].push(new_lens);
                    }
                }
            }
            _ => unreachable!(),
        }
    });

    let power = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(slot, lens)| (i + 1) * (slot + 1) * (lens.1 as usize))
                .sum::<usize>()
        })
        .sum::<usize>();

    Some(power)
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2023, 15);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2023, 15);
        assert_eq!(part_one(&input), Some(1320));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2023, 15);
        assert_eq!(part_two(&input), Some(145));
    }
}
