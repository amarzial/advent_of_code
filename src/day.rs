pub struct Day {
    pub year: u32,
    pub day: u32,
    pub input: String,
    pub part1: Option<String>,
    pub part2: Option<String>,
    pub time: std::time::Duration,
}

pub enum Part {
    First,
    Second,
    Both,
}

impl Day {
    fn load_input(&mut self) {
        self.input =
            std::fs::read_to_string(format!("./input/{:04}/d{:02}.txt", self.year, self.day))
                .unwrap_or(String::from(""));
    }

    pub fn new(year: u32, day: u32) -> Day {
        let mut d = Day {
            year,
            day,
            input: "".to_owned(),
            part1: Option::None,
            part2: Option::None,
            time: std::time::Duration::new(0, 0),
        };
        d.load_input();
        d
    }
}
