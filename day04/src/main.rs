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


fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
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