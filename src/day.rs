pub struct Day {
    pub year: u32,
    pub day: u32,
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
    fn load(&self) -> String {
        self.input = std::fs::read_to_string(format!("./y{04}/d{02}.txt"));
    }

    pub fn new(year: u32, day: u32) -> Day {
        Day {
            year,
            day,
            input: "".to_owned(),
            part1: Option::None,
            part2: Option::None,
        }
    }
}
