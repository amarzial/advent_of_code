use std::io::Write;

fn main_file() -> String {
    let mut s = String::new();
    for i in 1..26 {
        s += format!("pub mod d{:02};\n", i).as_str();
    }
    s
}

fn day_file() -> String {
    String::from(
        "use crate::day;

pub fn run(_d: &mut day::Day) -> bool {
    true
}
",
    )
}

fn main() {
    let y = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();

    {
        let mut f = std::fs::File::create(format!("./src/y{:04}.rs", y)).unwrap();
        f.write_all(main_file().as_bytes()).unwrap();
    }
    std::fs::create_dir(format!("./src/y{:04}", y)).unwrap();
    for d in 1..26 {
        let mut f = std::fs::File::create(format!("./src/y{:04}/d{:02}.rs", y, d)).unwrap();
        f.write_all(day_file().as_bytes()).unwrap();
    }
}
