use std::collections::HashMap;

use aoc::utils;

fn run(mut a: i32, mut b: i32) -> i32 {
    let mut score_a = 0;
    let mut score_b = 0;

    let mut die = 0;
    let mut roll = || {
        let mut res = 0;
        res += die + 1;
        die = (die + 1) % 100;
        res += die + 1;
        die = (die + 1) % 100;
        res += die + 1;
        die = (die + 1) % 100;
        res
    };
    a -= 1;
    b -= 1;
    let mut roll_count = 0;
    loop {
        let r1 = roll();
        roll_count += 3;
        a = (a + r1) % 10;
        score_a += a + 1;
        if score_a >= 1000 {
            break;
        }
        let r2 = roll();
        roll_count += 3;
        b = (b + r2) % 10;
        score_b += b + 1;
        if score_b >= 1000 {
            break;
        }
    }
    return i32::min(score_a, score_b) * roll_count;
}
use std::collections::BTreeMap;
type Rolls = BTreeMap<i32, i64>;

fn run2(
    lanci: &Rolls,
    posizione: [i32; 2],
    punteggi: [i64; 2],
    giocatore: usize,
    cache: &mut HashMap<([i32; 2], [i64; 2], usize), [i64; 2]>,
) -> [i64; 2] {
    let mut wins: [i64; 2] = [0, 0];
    match cache.get(&(posizione, punteggi, giocatore)) {
        Some(val) => {
            return *val;
        }
        None => {}
    }

    for (lancio, qta) in lanci.iter() {
        let mut posizione_current = posizione;
        let mut punteggio_current = punteggi;
        posizione_current[giocatore] = (posizione_current[giocatore] + lancio - 1) % 10 + 1;
        punteggio_current[giocatore] += posizione_current[giocatore] as i64;

        if punteggio_current[giocatore] >= 21 {
            wins[giocatore] += qta;
        } else {
            let res = run2(
                lanci,
                posizione_current,
                punteggio_current,
                (giocatore + 1) % 2,
                cache,
            );
            wins[0] += res[0] as i64 * qta;
            wins[1] += res[1] as i64 * qta;
        }
    }
    cache.insert((posizione, punteggi, giocatore), wins);
    return wins;
}

fn main() {
    let input = utils::read_list_parse(&utils::get_input(), |f| {
        f.chars()
            .skip("Player 1 starting position: ".len())
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    });

    let p1 = run(input[0], input[1]);
    println!("Part 1: {}", p1);

    let mut lanci_possibili = Rolls::new();
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let e = lanci_possibili.entry(a + b + c).or_insert(0 as i64);
                *e += 1;
            }
        }
    }

    let mut cache = HashMap::new();
    let p2 = run2(
        &lanci_possibili,
        [input[0], input[1]],
        [0, 0],
        0,
        &mut cache,
    );
    println!("{:?}", i64::max(p2[0], p2[1]));
}
