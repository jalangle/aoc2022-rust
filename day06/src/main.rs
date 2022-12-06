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

fn all_unique(chars : Vec<char>) -> bool {
    let mut has : Vec<char> = vec![];
    for c in chars {
        if has.contains(&c) {
            return false;
        }
        has.push(c);
    }
    return true;
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    for line in lines {
        let chars: Vec<char> = line.chars().collect::<Vec<_>>();
        let mut counter = 4;
        for window in chars.windows(counter) {
            if all_unique(window.to_vec()) {
                break;
            }
            counter += 1;
        }
        println!("{} {}", line, counter);
    }
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    for line in lines {
        let chars: Vec<char> = line.chars().collect::<Vec<_>>();
        let mut counter = 14;
        for window in chars.windows(counter) {
            if all_unique(window.to_vec()) {
                break;
            }
            counter += 1;
        }
        println!("{} {}", line, counter);
    }
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}