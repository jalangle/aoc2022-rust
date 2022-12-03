#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

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

fn value1(c: char) -> i32 {
    return match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => -1000000,
    }
}

fn both_contain1(compartment1: Vec<char>, compartment2: Vec<char>) -> Vec<char> {
    let mut both : Vec<char> = vec![];

    for c in &compartment1 {
        if compartment2.contains(&c) {
            both.push(*c)
        }
    }

    for c in &compartment2 {
        if compartment1.contains(&c) {
            both.push(*c)
        }
    }

    both.sort();
    both.dedup();

    return both
}

fn part1(path: &String) {
    println!("File: {path}");
    let rucksacks = file_to_lines(path);
    let mut scores : Vec<i32> = vec![];
    for r in rucksacks {
        let size = r.len() / 2;
        let compartment1 : Vec<char> = r[0..size].chars().collect();
        let compartment2 : Vec<char> = r[size..].chars().collect();
        let overlap = both_contain1(compartment1, compartment2);
        let score : i32 = overlap.iter().map(|c| value1(*c)).sum();
        scores.push(score);
    }
    let score : i32 = scores.iter().sum();
    println!("{}", score);
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part1(&file),
        None => println!("No file"),
    }
}