mod day2;
use std::time::{Instant};

fn main() {
    let now = Instant::now();

    day2::solution();
    println!("{}ms", now.elapsed().as_millis());
    let now = Instant::now();

    day2::solution2();
    println!("{}ms", now.elapsed().as_millis());
}
