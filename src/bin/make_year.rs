fn main_file() -> String {
    let mut s = String::new();
    for i in 1..26 {
        s += format!("pub mod d{:02}\n", i).as_str();
    }
    s
}

fn day_file(day: i32) -> String {
    String::from(
        "use crate::day;

pub fn run(_d: &mut day::Day, _which: day::Part) -> bool {
    true
}
",
    )
}

fn main() {
    print!("{}", main_file());
}
