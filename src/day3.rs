use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solution(){
    let reader = BufReader::new(File::open("./res/day3.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();
    let m : Vec<Vec<bool>> = rows.iter().map(|r| r.chars().map(|t| t == '#').collect()).collect();
    println!("{}",calc_trees(&m, 3, 1));
}

pub fn solution2(){
    let reader = BufReader::new(File::open("./res/day3.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();
    let m : Vec<Vec<bool>> = rows.iter().map(|r| r.chars().map(|t| t == '#').collect()).collect();
    let mul : i64 = calc_trees(&m, 1, 1)*calc_trees(&m, 3, 1)*calc_trees(&m, 5, 1)*calc_trees(&m, 7, 1)*calc_trees(&m, 1, 2);
    println!("{}", mul);
}

fn calc_trees(mp: &Vec<Vec<bool>>, right: usize, down : usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < mp.len(){
        if mp[y][x] { 
            trees += 1;
        }
        x += right;
        x %= mp[0].len();
        y += down;
    }
    return trees;
}