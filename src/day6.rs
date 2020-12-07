use std::fs::File;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
pub fn solution(){
    let reader = BufReader::new(File::open("./res/day6.in").expect("Couldn't open file"));
    let rows: Vec<String> = reader.lines().map(|l| l.expect("Expected line")).collect();

    let mut hs : HashSet<char> = HashSet::new();
    let mut sum = 0;
    for row in rows{
        if row.len() == 0{
            sum += hs.len();
            hs.clear();
        }
        row.chars().for_each(|c| {
            hs.insert(c);
        });
    }
    sum += hs.len();

    println!("{}", sum);
}

pub fn solution2(){
    let contents = fs::read_to_string("./res/day6.in").expect("Something went wrong reading the file");
    let groups :  Vec<Vec<HashSet<char>>> = contents.split("\r\n\r\n").map(|g| g.lines().map(|s| s.to_string().chars().collect()).collect()).collect();
    let sum : usize = groups.iter().map(|v| intersect_sets(&v).len()).sum();
    println!("{}", sum);
}

fn intersect_sets(sets : &Vec<HashSet<char>>) -> HashSet<char>{
    let mut return_set = sets.iter().next().unwrap().clone();
    for set in sets{
        return_set = return_set.intersection(set).map(|c| *c).collect();
    }
    return_set
}
