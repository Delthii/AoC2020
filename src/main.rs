use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("./res/in.txt")?);
    let mut sum = 0;
    for line in reader.lines() {
        let mut fuel = line.unwrap().parse::<i32>().unwrap();
        while fuel > 0 {
            fuel = fuel / 3 - 2;
            if fuel > 0 {
                sum += fuel;
            }
        }
    }        
    println!("{}", sum);
    Ok(())
}
