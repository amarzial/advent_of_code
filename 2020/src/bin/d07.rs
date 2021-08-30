use aoc2020::utils;
use std::cell::RefCell;
use std::collections::HashMap;

struct Bag {
    name: String,
    has_gold: bool,
    contained_bags: u64,
    bags: Vec<(u64, String)>,
}

impl Bag {
    fn new(name: &str, has_gold: bool, contained_bags: u64, bags: Vec<(u64, String)>) -> Bag {
        return Bag {
            name: String::from(name),
            has_gold,
            contained_bags,
            bags,
        };
    }
}

type Baglist = HashMap<String, RefCell<Bag>>;

fn parse_bag(line: &str) -> Bag {
    let mut parts = line.split(" bags contain ");
    let bag_name = parts.next().unwrap();
    let mut b = Bag::new(bag_name, false, 0, Vec::new());
    for part in parts.next().unwrap().split(", ") {
        if !part.starts_with("no") {
            let start = part.find(" ").unwrap();
            let end = part.find(" bag").unwrap();

            b.bags.push((
                part.get(0..start).unwrap().parse::<u64>().unwrap(),
                String::from(part.get(start + 1..end).unwrap()),
            ));
        }
    }
    return b;
}

fn count_bag(bags: &Baglist, bag_name: &str) -> (u64, bool) {
    let mut bag = bags.get(bag_name).unwrap().borrow_mut();

    if bag.contained_bags == 0 {
        let mut count: u64 = 0;
        let mut gold = bag_name == "shiny gold";
        if bag.bags.len() > 0 {
            for contained_bag in bag.bags.iter() {
                let info = count_bag(bags, &contained_bag.1);
                count += contained_bag.0 * info.0;
                gold = gold || info.1;
            }
        }
        bag.contained_bags = 1 + count;
        bag.has_gold = gold;
    }
    return (bag.contained_bags, bag.has_gold);
}

fn main() {
    let list = utils::read_list_parse("inputs/d07.txt", parse_bag);
    let mut baglist: Baglist = HashMap::new();
    let mut bag_names = Vec::new();
    for b in list.into_iter() {
        bag_names.push(String::from(&b.name));
        baglist.insert(String::from(&b.name), RefCell::new(b));
    }

    let mut count_gold = 0;
    for name in bag_names.iter() {
        let info = count_bag(&baglist, name);
        if info.1 == true {
            count_gold += 1;
        }
    }
    println!("Part 1: {}", count_gold - 1);

    println!(
        "Part 2: {}",
        baglist.get("shiny gold").unwrap().borrow().contained_bags - 1
    );
}
