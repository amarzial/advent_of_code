use crate::day;
use crate::utils;

pub fn run(d: &mut day::Day) -> bool {
    let lines = utils::read_list::<String>(&d.input);
    for line in lines.iter() {
        print!("{}", line);
    }
    true
}
