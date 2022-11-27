use std::process::Command;

fn main() {
    for y in 2015..=2022 {
        for d in 1..=25 {
            let out = Command::new("cargo")
                .args(["run", "--release", "--bin", &format!("{:04}_{:02}", y, d)])
                .output()
                .unwrap();

            if out.stdout.len() > 0 {
                print!(
                    "Year {} - Day {:02}:\n{}",
                    y,
                    d,
                    String::from_utf8(out.stdout).unwrap()
                );
            }
        }
    }
}
