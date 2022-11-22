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

    pub fn set_part_1(&mut self, value: String) {
        self.part1 = Some(value)
    }

    pub fn set_part_2(&mut self, value: String) {
        self.part2 = Some(value)
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

    pub fn print_result(&self) {
        let p1 = match &self.part1 {
            Some(s) => s.to_owned(),
            None => String::from(""),
        };
        let p2 = match &self.part2 {
            Some(s) => s.to_owned(),
            None => String::from(""),
        };

        if !self.part1.is_some() && !self.part2.is_some() {
            print!("Year {}, Day {}: Missing\n", self.year, self.day);
        } else {
            print!(
                "Year {}, Day {}: {}.{:06}s\n  part 1: {}\n  part 2: {}\n",
                self.year,
                self.day,
                self.time.as_secs(),
                self.time.subsec_micros(),
                p1,
                p2
            );
        }
    }

    pub fn stars(&self) -> i32 {
        let mut s = 0;
        if self.part1.is_some() {
            s += 1;
        }
        if self.part2.is_some() {
            s += 1;
        }
        s
    }
}
