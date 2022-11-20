pub struct Day {
    pub input: String,
    pub part1: Option<String>,
    pub part2: Option<String>,
}

pub enum Part {
    First,
    Second,
    Both,
}

impl Day {
    pub fn new() -> Day {
        Day {
            input: "".to_owned(),
            part1: Option::None,
            part2: Option::None,
        }
    }
}
