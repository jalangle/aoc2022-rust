#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;

pub fn file_to_lines(path: &String) -> Vec<String> {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => {}
    };

    let lines : Vec<String> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
    lines
}

fn set_from_string(range: &String) -> HashSet<i32> {
    let parts : Vec<i32> = range.split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut set : HashSet<i32>  = HashSet::new();
    for i in parts[0]..parts[1]+1 {
        set.insert(i);
    }
    return set 
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
    let mut count : i32 = 0;
    for l in lines {
        let elves = l.split(",").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let first_range = set_from_string(&elves[0]);
        let second_range = set_from_string(&elves[1]);

        let intersection = first_range.intersection(&second_range).collect::<Vec<&i32>>();
        if first_range.len() == intersection.len() || second_range.len() == intersection.len() {
            count += 1;
        }
    }

    println!("{}", count);
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
    let mut count : i32 = 0;
    for l in lines {
        let elves = l.split(",").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let first_range = set_from_string(&elves[0]);
        let second_range = set_from_string(&elves[1]);

        let intersection = first_range.intersection(&second_range).collect::<Vec<&i32>>();
        if intersection.len() > 0 {
            count += 1;
        }
    }

    println!("{}", count);
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}