use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

pub fn solution(){
    let reader = BufReader::new(File::open("./res/day5.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();

    let res = rows.iter()
        .map(|s| s.chars().map(|c| make_binary(c))
        .collect::<String>())
        .map(|s| make_tuple(&s))
        .map(|(row, col)| row*8 + col)
        .max()
        .unwrap();

    println!("{}", res);
}

fn make_binary(s: char) -> char{
    match s{
        'F' | 'L' => '0',
        'B' | 'R' => '1',
        _ => panic!("")
    }
}

fn make_tuple(s: &String) -> (i32, i32){
    (i32::from_str_radix(&s[..7], 2).unwrap(), i32::from_str_radix(&s[7..], 2).unwrap())
}

pub fn solution2(){
    let reader = BufReader::new(File::open("./res/day5.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();
    let res: HashSet<_> = rows.iter()
        .map(|s| s.chars().map(|c| make_binary(c))
        .collect::<String>()).map(|s| make_tuple(&s))
        .collect();
    for row in 2..110{
        for col in 0..8{
            if !res.contains(&(row, col)){
                println!("{}", row*8+col);
            }
        }
    }
}
