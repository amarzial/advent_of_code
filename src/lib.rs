pub mod day;
pub mod grid;
pub mod utils;

#[macro_export]
macro_rules! solve {
    ($part: expr, $function:ident, $input: expr) => {{
        fn print_result<T: std::fmt::Display, P>(func: impl FnOnce(P) -> Option<T>, input: P) {
            let timer = std::time::Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            match result {
                Some(result) => {
                    println!("  {} (elapsed: {:.2?})", result, elapsed);
                }
                None => {
                    println!("  not solved.")
                }
            }
        }

        println!("Part {}", $part);
        print_result($function, $input);
    }};
}
