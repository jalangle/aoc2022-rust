#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ops::{Add, Sub};
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

    let lines : Vec<String> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
    lines
}

fn parse_line(line: &String) -> (String, i32) {
    let parts : Vec<String> = line.split(' ').map(|x| x.to_string()).collect();
    return(parts[0].clone(), parts[1].parse::<i32>().unwrap());
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Point {
    fn abuts(&self, other : &Point) -> bool {
        let diff : Point = *self - *other;
        return -1 <= diff.x && diff.x <= 1 && -1 <= diff.y && diff.y <= 1
    }
}

fn direction_to_diff(direction: &str) -> Point {
    match direction {
        "U" => return Point{x: 0, y: 1},
        "D" => return Point{x: 0, y: -1},
        "L" => return Point{x: -1, y: 0},
        "R" => return Point{x: 1, y: 0},
        &_ => return Point{x: -1000000, y: -1000000},
    }
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let mut head = Point {x: 0, y: 0};
    let mut tail = Point {x: 0, y: 0};
    let mut tail_points : HashSet<Point> = HashSet::new();

    for l in lines {
        let (direction, distance) = parse_line(&l);
        //println!("{} {}", direction, distance);

        let diff = direction_to_diff(&direction);

        for i in 0..distance {
            let tail_diff = head - tail;
            head = head + diff;
            if !tail.abuts(&head) {
                tail = tail + tail_diff;
            }
            tail_points.insert(tail);
            //println!("{:?} | {:?}", head, tail);
        }
    }

    println!("{}", tail_points.len());
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