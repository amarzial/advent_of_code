use std::collections::HashMap;

use aoc::utils;

fn get_state(string: &String) -> HashMap<String, u64> {
    let mut state = HashMap::new();
    for pair in string.as_bytes().windows(2) {
        let p = String::from_utf8_lossy(pair).to_string();
        let c = state.entry(p).or_insert(0);
        *c += 1;
    }
    return state;
}

fn pass(state: &mut HashMap<String, u64>, pairs: &HashMap<String, char>) {
    let checks = state.clone();

    for c in checks.iter() {
        *state.get_mut(c.0).unwrap() -= c.1;
        let new = pairs[c.0];
        let mut citer = c.0.chars();
        let mut c1 = String::from(citer.next().unwrap());
        c1.push(new);
        let mut c2 = String::from(new);
        c2.push(citer.next().unwrap());

        let new1 = state.entry(c1).or_insert(0);
        *new1 += *c.1;
        let new2 = state.entry(c2).or_insert(0);
        *new2 += *c.1;
    }
}

fn count(state: &HashMap<String, u64>) -> u64 {
    let mut count = HashMap::new();
    for (k, v) in state.iter() {
        let mut c = k.chars();
        let c1 = count.entry(c.next().unwrap()).or_insert(0);
        *c1 += *v;
        let c2 = count.entry(c.next().unwrap()).or_insert(0);
        *c2 += *v;
    }

    let mut min = u64::MAX;
    let mut max = 0;
    for n in count.values() {
        min = min.min(*n);
        max = max.max(*n);
    }
    return (max / 2 + max % 2) - (min / 2 + min % 2);
}

fn main() {
    let input: Vec<String> = utils::read_list(&utils::get_input());
    let mut input_iter = input.iter();
    let string = input_iter.next().unwrap().to_string();
    let mut pairs: HashMap<String, char> = HashMap::new();
    input_iter.next();
    for pair in input_iter {
        let mut sides = pair.split(" -> ");
        pairs.insert(
            sides.next().unwrap().to_string(),
            sides.next().unwrap().chars().next().unwrap(),
        );
    }

    let mut state = get_state(&string);

    for _i in 0..10 {
        pass(&mut state, &pairs);
    }
    println!("Part 1: {:?}", count(&state));

    for _i in 10..40 {
        pass(&mut state, &pairs);
    }
    println!("Part 2: {:?}", count(&state));
}
