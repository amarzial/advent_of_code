use std::collections::HashSet;

use aoc::utils;

enum Fold {
    X(usize),
    Y(usize),
}

fn fold(grid: &mut HashSet<(usize, usize)>, fold: &Fold) {
    let mut points = Vec::new();
    let n;
    let h;
    match fold {
        Fold::X(pos) => {
            h = false;
            n = *pos;
        }
        Fold::Y(pos) => {
            h = true;
            n = *pos;
        }
    }
    for p in grid.iter() {
        if (h && p.1 > n) || (!h && p.0 > n) {
            points.push(p.clone());
        }
    }
    for p in points.iter() {
        let mut new = p.clone();
        if h {
            new.1 = 2 * n - new.1;
        } else {
            new.0 = 2 * n - new.0;
        }
        grid.remove(p);
        grid.insert(new);
    }
}

fn print(grid: &HashSet<(usize, usize)>) {
    let mut width = 0;
    let mut height = 0;
    for elm in grid.iter() {
        width = elm.0.max(width);
        height = elm.1.max(height);
    }
    width += 1;
    height += 1;
    let mut v = vec![' '; width * height];
    for elm in grid.iter() {
        v[elm.0 + (elm.1 * width)] = '#';
    }
    for w in v.chunks(width) {
        println!("{}", w.iter().collect::<String>());
    }
    println!("");
}

fn main() {
    let input: Vec<String> = utils::read_list(&utils::get_input());

    let mut grid: HashSet<(usize, usize)> = HashSet::new();
    let mut folds = Vec::new();
    for line in input.iter() {
        if line.starts_with("f") {
            let mut fold = line.strip_prefix("fold along ").unwrap().split("=");
            let dir = fold.next().unwrap();
            let pos = fold.next().unwrap().parse::<usize>().unwrap();
            folds.push(match dir {
                "x" => Fold::X(pos),
                "y" => Fold::Y(pos),
                _ => panic!(),
            })
        } else if line.len() > 1 {
            let mut s = line.split(",");
            grid.insert((
                s.next().unwrap().parse::<usize>().unwrap(),
                s.next().unwrap().parse::<usize>().unwrap(),
            ));
        }
    }

    let mut f = folds.iter();
    fold(&mut grid, f.next().unwrap());
    println!("Part 1: {}", grid.len());

    for fl in f {
        fold(&mut grid, fl);
    }
    println!("Part 2:");
    print(&grid);
}
