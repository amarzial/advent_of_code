use aoc2020::utils;

struct Point {
    x: usize,
    y: usize,
}

fn slide(forest: &Vec<Vec<u8>>, direction: &Point) -> u64 {
    let mut position = Point { x: 0, y: 0 };
    let width = forest[0].len();

    let mut trees = 0;
    while position.y < forest.len() {
        if forest[position.y][position.x % width] == b'#' {
            trees += 1;
        }
        position.x += direction.x;
        position.y += direction.y;
    }

    println!("{}", trees);
    return trees;
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or(String::from("inputs/d03.txt"));
    let list = utils::read_list_parse(&filename, |line| {
        return String::from(line).as_bytes().to_vec();
    });

    let direction = Point { x: 3, y: 1 };

    let trees = slide(&list, &direction);

    println!("Part 1: {}", trees);

    let slopes = [
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];

    let mut trees: u64 = 1;
    for i in 0..slopes.len() {
        trees *= slide(&list, &slopes[i]);
    }
    println!("Part 2: {}", trees);
}
