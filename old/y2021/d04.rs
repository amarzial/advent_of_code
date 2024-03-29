use crate::day;
use crate::utils;

use std::collections::HashMap;

struct Board {
    numbers: HashMap<u8, usize>,
    rows: [[u8; 5]; 5],
    columns: [[u8; 5]; 5],
    bingo: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            numbers: HashMap::new(),
            rows: [[0; 5]; 5],
            columns: [[0; 5]; 5],
            bingo: false,
        }
    }

    fn check_bingo(&self, row: usize, col: usize) -> bool {
        let mut cnt = 0;
        for c in self.rows[row] {
            cnt += c;
        }
        if cnt == 5 {
            return true;
        }
        cnt = 0;
        for r in self.columns[col] {
            cnt += r;
        }
        if cnt == 5 {
            return true;
        }
        return false;
    }

    fn set(&mut self, number: u8) -> bool {
        match self.numbers.get(&number) {
            Some(pos) => {
                let row = pos / 5;
                let col = pos % 5;
                self.rows[row][col] = 1;
                self.columns[col][row] = 1;
                self.check_bingo(row, col)
            }
            None => false,
        }
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;
        for (n, pos) in self.numbers.iter() {
            if self.rows[*pos / 5][*pos % 5] == 0 {
                sum += *n as u32;
            }
        }
        sum
    }
}

fn parse_boards<'a>(lines: impl Iterator<Item = &'a String>) -> Vec<Board> {
    let mut boards = Vec::new();
    let mut b = Board::new();
    let mut count = 0;
    for line in lines {
        if line.len() == 0 {
            boards.push(b);
            b = Board::new();
            count = 0;
        } else {
            for i in line
                .split_whitespace()
                .map(|x| -> u8 { x.parse::<u8>().unwrap() })
            {
                b.numbers.insert(i, count);
                count += 1;
            }
        }
    }
    boards.push(b);
    boards
}

pub fn run(d: &mut day::Day) -> bool {
    let list: Vec<String> = utils::read_list(&d.input);

    let mut lines = list.iter();
    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| -> u8 { x.parse::<u8>().unwrap() })
        .collect();
    lines.next();

    let mut boards = parse_boards(lines);

    let mut result = 0;
    let mut result2 = 0;
    for n in numbers.iter() {
        for board in boards.iter_mut() {
            if board.set(*n) && !board.bingo {
                board.bingo = true;
                result2 = board.sum() * *n as u32;
                if result == 0 {
                    result = result2;
                }
            }
        }
    }

    d.set_part_1(result.to_string());
    d.set_part_2(result2.to_string());
    true
}
