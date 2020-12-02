use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solution(){
    let reader = BufReader::new(File::open("./res/day2.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();

    let valid = rows
        .iter().map(|r| parse(r))
        .map(|inp| (inp.min, inp.max, inp.password.iter().filter(|letter| **letter == inp.letter).count()))
        .filter(|(min, max, res)| res >= min && res <= max)
        .count();

    println!("{}", valid);
}

pub fn solution2(){
    let reader = BufReader::new(File::open("./res/day2.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();

    let valid = rows
        .iter().map(|r| parse(r))
        .filter(|inp| (inp.password[inp.min - 1] == inp.letter) ^ (inp.password[inp.max - 1] == inp.letter))
        .count();
    
    println!("{}", valid);
}

struct Input{
    min: usize,
    max: usize,
    letter: char,
    password: Vec<char>
}

fn parse(row: &String) -> Input{
    let v: Vec<&str> = row.split(|c| c == ':' || c == ' ' || c == '-').collect();
    let mi = v[0].parse::<usize>().unwrap(); 
    let ma = v[1].parse::<usize>().unwrap();
    let c = v[2].chars().nth(0).unwrap();
    let pw: Vec<char> = v[4].chars().collect();
    Input{
        min: mi,
        max: ma,
        letter: c,
        password: pw
    }
}

