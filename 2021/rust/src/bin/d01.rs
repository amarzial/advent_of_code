use aoc::utils;

fn main() {
    let filename = utils::get_input();
    let sonar: Vec<u32> = utils::read_list(&filename);

    let mut last = *sonar.get(0).unwrap();
    let mut cnt = 0;
    for depth in sonar.iter() {
        if *depth > last {
            cnt += 1;
        }
        last = *depth;
    }

    let mut cnt2 = 0;
    let mut last2: u32 = sonar.windows(3).next().unwrap().iter().sum();
    for win in sonar.windows(3).skip(1) {
        let s: u32 = win.iter().sum();
        if s > last2 {
            cnt2 += 1;
        }
        last2 = s;
    }
    println!("Part 1: {}", cnt);
    println!("Part 2: {}", cnt2);
}
