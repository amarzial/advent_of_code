use aoc::utils;

fn ratings<'a>(list: impl Iterator<Item = &'a Box<String>>, length: usize) -> Vec<i32> {
    let mut bits = Vec::new();
    bits.resize(length, 0);
    for number in list {
        let s = number.as_bytes();
        for i in 0..length {
            if s[i] == b'1' {
                bits[i] += 1;
            }
        }
    }
    bits
}

fn main() {
    let list: Vec<Box<String>> =
        utils::read_list_parse(&utils::get_input(), |line| -> Box<String> {
            return Box::new(String::from(line));
        });
    let length = 12;
    let bits = ratings(list.iter(), length);

    let half = list.len() / 2;
    let mut gamma = 0;
    for i in 0..length {
        gamma <<= 1;
        gamma |= if bits[i] as usize > half { 1 } else { 0 };
    }
    let epsilon = !gamma & (2u32.pow(length as u32) - 1);

    println!("Part 1: {}", gamma * epsilon);

    //part 2
    let mut ox = list.clone();
    let mut i = 0;
    while ox.len() > 1 {
        let rate = ratings(ox.iter(), i + 1);
        let cnt = rate[rate.len() - 1];
        let compare = if cnt >= ox.len() as i32 - cnt {
            b'1'
        } else {
            b'0'
        };

        ox = ox
            .into_iter()
            .filter(|x| -> bool { (*x).as_bytes()[i] == compare })
            .collect();
        i += 1;
    }
    let mut co = list.clone();
    i = 0;
    while co.len() > 1 {
        let rate = ratings(co.iter(), i + 1);
        let cnt = rate[rate.len() - 1];
        let compare = if cnt < co.len() as i32 - cnt {
            b'1'
        } else {
            b'0'
        };

        co = co
            .into_iter()
            .filter(|x| -> bool { (*x).as_bytes()[i] == compare })
            .collect();
        i += 1;
    }
    println!(
        "Part 2: {}",
        i32::from_str_radix(&ox[0], 2).unwrap() * i32::from_str_radix(&co[0], 2).unwrap()
    );
}
