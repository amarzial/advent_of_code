use crate::day;

type Target = ((i32, i32), (i32, i32));

fn get_input(input: &str) -> Target {
    let mut str = String::from(input);
    str.pop();
    let s = str.strip_prefix("target area: ").unwrap();
    let mut parts = s.split(", ");
    let px = parts.next().unwrap();
    let py = parts.next().unwrap();
    let x = px.strip_prefix("x=").unwrap().split("..");
    let y = py.strip_prefix("y=").unwrap().split("..");

    let mut out = ((0, 0), (0, 0));
    let mut outx = x.map(|e| e.parse::<i32>().unwrap());
    let mut outy = y.map(|e| e.parse::<i32>().unwrap());

    out.0 .0 = outx.next().unwrap();
    out.0 .1 = outx.next().unwrap();
    out.1 .0 = outy.next().unwrap();
    out.1 .1 = outy.next().unwrap();
    out
}

fn do_run(initial: (i32, i32), target: Target) -> Option<i32> {
    let min_x = target.0 .0.min(target.0 .1);
    let max_x = target.0 .0.max(target.0 .1);
    let min_y = target.1 .0.min(target.1 .1);
    let max_y = target.1 .0.max(target.1 .1);

    let mut pos = (0, 0);
    let mut v = initial;
    let mut max_height = i32::MIN;
    while pos.0 <= max_x && pos.1 >= min_y {
        pos.0 += v.0;
        pos.1 += v.1;
        max_height = max_height.max(pos.1);
        if pos.0 >= min_x && pos.0 <= max_x && pos.1 >= min_y && pos.1 <= max_y {
            return Some(max_height);
        }

        v.0 = 0.max(v.0 - 1);
        v.1 -= 1;
    }
    return None;
}

pub fn run(d: &mut day::Day) -> bool {
    let target = get_input(&d.input);

    let vx = i32::max(target.0 .0, target.0 .1);
    let vy = i32::min(target.1 .0, target.1 .1).abs();

    let mut max = 0;
    let mut cnt = 0;
    for y in -(vy + 1)..=vy {
        for x in 1..=vx {
            match do_run((x, y), target) {
                Some(height) => {
                    max = max.max(height);
                    cnt += 1;
                }
                None => {}
            }
        }
    }

    d.set_part_1(max.to_string());
    d.set_part_2(cnt.to_string());
    true
}
