use std::io::Read;

use aoc::utils;

struct Stream {
    data: Vec<u8>,
    pos: usize,
    version_sum: u64,
}

impl Stream {
    fn new(data: Vec<u8>) -> Stream {
        Stream {
            data,
            pos: 0,
            version_sum: 0,
        }
    }

    fn get_data(&mut self, size: usize) -> u64 {
        let start = self.pos / 4;
        let end = (self.pos + size - 1) / 4 + 1;
        let offset = self.pos % 4;
        let mut buf = [0; 16];
        (&self.data[start..end]).read(&mut buf).unwrap();

        self.pos += size;
        let mut out: u64 = 0;
        for v in buf {
            out <<= 4;
            out |= v as u64 & 0xf;
        }
        out <<= offset;
        out >>= 64 - size;
        out &= u64::MAX >> (64 - size);
        return out;
    }

    fn next(&mut self) -> u64 {
        let v = self.get_data(3);
        self.version_sum += v;
        let t = self.get_data(3);
        if t == 4 {
            //literal

            let mut val = 0;
            loop {
                let group = self.get_data(5);
                val <<= 4;
                val |= group & 0xf;
                if group & 0b10000 == 0 {
                    break;
                }
            }
            return val as u64;
        } else {
            let mut results = vec![];
            let id = self.get_data(1);
            if id == 0 {
                let len = self.get_data(15);
                let pos = self.pos;
                while self.pos < pos + len as usize {
                    results.push(self.next() as u64);
                }
            } else {
                let cnt = self.get_data(11);
                for _i in 0..cnt {
                    results.push(self.next() as u64);
                }
            }
            let mut iter = results.into_iter();
            match t {
                0 => iter.sum::<u64>(),
                1 => iter.fold(1, |tot, curr| tot * curr),
                2 => iter.min().unwrap(),
                3 => iter.max().unwrap(),
                5 => {
                    let base = iter.next().unwrap();
                    if base > iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let base = iter.next().unwrap();
                    if base < iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let base = iter.next().unwrap();
                    if base == iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            }
        }
    }
}

fn main() {
    let mut input = std::fs::read_to_string(&utils::get_input()).unwrap();
    input.pop();
    let mut stream = Stream::new(
        input
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect(),
    );

    let p2 = stream.next();
    println!("Part 1: {}", stream.version_sum);
    println!("Part 2: {}", p2);
}
