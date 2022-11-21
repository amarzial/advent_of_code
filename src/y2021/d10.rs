use crate::day;
use crate::utils;

enum Status {
    Complete,
    Incomplete(u64),
    Error(u64),
}

fn calc_incomplete(stack: &mut Vec<char>) -> u64 {
    let mut score: u64 = 0;
    while stack.len() > 0 {
        let c = stack.pop().unwrap();
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    }
    return score;
}

fn check_line(line: &str) -> Status {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => ' ',
                });
            }
            ')' | ']' | '}' | '>' => {
                let current = *stack.last().unwrap_or(&' ');
                if current == c {
                    stack.pop().unwrap();
                } else {
                    return Status::Error(match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    });
                }
            }
            _ => (),
        }
    }
    if stack.len() > 0 {
        return Status::Incomplete(calc_incomplete(&mut stack));
    }
    return Status::Complete;
}
pub fn run(d: &mut day::Day) -> bool {
    let input: Vec<String> = utils::read_list(&d.input);

    let mut error = 0;
    let mut scores = Vec::new();
    for l in input.iter() {
        match check_line(l) {
            Status::Error(v) => {
                error += v;
            }
            Status::Incomplete(v) => {
                scores.push(v);
            }
            _ => {}
        }
    }
    d.set_part_1(error.to_string());
    scores.sort();
    d.set_part_2((scores[scores.len() / 2]).to_string());
    true
}
