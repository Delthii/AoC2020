mod day3;
use std::time::{Instant};

fn main() {
    let now = Instant::now();

    day3::solution();
    println!("{}ms", now.elapsed().as_millis());
    let now = Instant::now();

    day3::solution2();
    println!("{}ms", now.elapsed().as_millis());
}
