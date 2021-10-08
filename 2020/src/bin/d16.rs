use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

struct Range {
    start: i32,
    end: i32,
}

type Ticket = Vec<i32>;

impl Range {
    fn new(str: &str) -> Range {
        let cursor = str.find('-').unwrap();
        let range = Range {
            start: str.get(0..cursor).unwrap().parse().unwrap(),
            end: str.get((cursor + 1)..).unwrap().parse().unwrap(),
        };
        return range;
    }
}

fn read(filename: &str) -> (HashMap<String, (Range, Range)>, Ticket, Vec<Ticket>) {
    let f = std::fs::File::open(filename).unwrap();
    let mut buff = std::io::BufReader::new(f);
    let mut s = String::new();

    let mut rules = HashMap::new();
    while (s.clear() == ()) && buff.read_line(&mut s).unwrap() > 0 {
        s.pop();
        if s.len() == 0 {
            break;
        }
        let mut cursor = s.find(':').unwrap();
        let name = String::from(s.get(0..cursor).unwrap());
        cursor += 2;
        let cursor2 = cursor + s.get(cursor..).unwrap().find(" or ").unwrap();
        let range1 = Range::new(s.get(cursor..cursor2).unwrap());
        let range2 = Range::new(s.get((cursor2 + 4)..).unwrap());
        rules.insert(name, (range1, range2));
    }

    buff.read_line(&mut s).unwrap();
    s.clear();
    buff.read_line(&mut s).unwrap();
    s.pop();
    let ticket: Ticket = s.split(',').map(|e| e.parse::<i32>().unwrap()).collect();

    buff.read_line(&mut s).unwrap();
    s.clear();
    buff.read_line(&mut s).unwrap();
    s.clear();

    let mut tickets = Vec::new();
    while (s.clear() == ()) && buff.read_line(&mut s).unwrap() > 0 {
        s.pop();
        if s.len() == 0 {
            break;
        }
        let t: Ticket = s.split(',').map(|e| e.parse::<i32>().unwrap()).collect();
        tickets.push(t);
    }

    return (rules, ticket, tickets);
}

fn validate(
    ticket: &Ticket,
    rules: &HashMap<String, (Range, Range)>,
    fields: &mut HashMap<usize, HashSet<&str>>,
) -> i32 {
    let mut sum = 0;
    let mut column = 0;
    let mut removing = Vec::with_capacity(fields.len());
    for val in ticket.iter() {
        let mut valid = false;
        removing.clear();
        for (name, rule) in rules.iter() {
            if ((*val >= rule.0.start) && (*val <= rule.0.end))
                || ((*val >= rule.1.start) && (*val <= rule.1.end))
            {
                valid = true;
            } else {
                removing.push(name);
            }
        }
        if !valid {
            sum += val;
        } else {
            for f in removing.iter() {
                fields
                    .get_mut(&(column as usize))
                    .unwrap()
                    .remove(f.get(..).unwrap());
            }
        }
        column += 1;
    }
    return sum;
}

fn main() {
    let (rules, ticket, tickets) = read("inputs/d16.txt");

    let mut fields: HashMap<usize, HashSet<&str>> = HashMap::new();
    for col in 0..rules.len() {
        fields.insert(col, rules.keys().map(|s| s.get(..).unwrap()).collect());
    }

    let mut total = 0;
    for t in tickets.iter() {
        total += validate(&t, &rules, &mut fields);
    }

    println!("Part 1: {}", total);
    let mut mapping = HashMap::new();
    let mut done = false;
    while !done {
        done = true;
        for (col, f) in fields.iter() {
            if f.len() == 1 {
                let v = *f.iter().next().unwrap();
                mapping.insert(v, *col);
                for int in fields.values_mut() {
                    int.remove(v);
                }
                done = false;
                break;
            }
        }
    }

    let mut total2 = 1 as usize;
    for (k, v) in mapping.iter() {
        if k.find("departure").is_some() {
            total2 *= *ticket.get(*v).unwrap() as usize;
        }
    }
    println!("Part 2: {:?}", total2);
}
