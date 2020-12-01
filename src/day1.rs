use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

pub fn solution(){
    let reader = BufReader::new(File::open("./res/day1.in").expect("Couldn't open file"));
    let mut s = HashSet::new();
    for line in reader.lines() {
        let cash = line.expect("Expected line").parse::<i32>().expect("Failed parsing int");
        let delta = 2020 - cash;
        if s.contains(&delta){
            println!("{}", cash*delta)
        }else{
            s.insert(cash);
        }
    }
}

pub fn solution2(){
    let reader = BufReader::new(File::open("./res/day1.in").expect("Couldn't open file"));
    let mut numbers = Vec::new();

    for line in reader.lines() {
        numbers.push(line.expect("Expected line").parse::<i32>().expect("Failed parsing int"));
    }

    numbers.sort();

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let x = 2020 - numbers[i] - numbers[j];
            let mut k = numbers.len() - 1;
            while k > j && x < numbers[k] {
                k -= 1;
            }
            if k > j && x == numbers[k] {
                println!("{}", numbers[i]*numbers[j]*numbers[k])
            }
        }
    }
}