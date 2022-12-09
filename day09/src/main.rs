#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::ops::{Add, Sub};
use std::collections::HashSet;

fn file_to_lines() -> Vec<String> {
    let bytes = include_bytes!("input1.txt");
    let lines = String::from_utf8_lossy(bytes)
                .split("\n")
                .collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
    return lines
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

fn part1() {
    let lines = file_to_lines();

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

    println!("{}", tail_points.len()); // test1 = 13, test2 = 88, input1 = 5878
}

fn part2() {
    let error_point = Point{x:-10000,y:-10000};

    let lines = file_to_lines();


    let mut tail_points : HashSet<Point> = HashSet::new();

    let cknots = 10;
    let mut rope : Vec<Point> = vec![];
    for x in 0..cknots {
        rope.push(Point{x:0,y:0})
    }

    for l in lines {

        let (direction, distance) = parse_line(&l);
        let initial = direction_to_diff(&direction);

        for _ in 0..distance {
            rope[0] = rope[0] + initial;

            assert!(rope.len() > 1);
            for k in 1..rope.len() {

                if rope[k].abuts(&rope[k-1]) { // if this knot is adjacent to the previous knot, stop processing
                    break;
                }

                let change = match rope[k-1] - rope[k] {
                    Point{x:-1,y:2} => Point{x:-1,y:1},
                    Point{x:0,y:2} => Point{x:0,y:1},
                    Point{x:1,y:2} => Point{x:1,y:1},

                    Point{x:-1,y:-2} => Point{x:-1,y:-1},
                    Point{x:0,y:-2} => Point{x:0,y:-1},
                    Point{x:1,y:-2} => Point{x:1,y:-1},

                    Point{x:2,y:-1} => Point{x:1,y:-1},
                    Point{x:2,y:0} => Point{x:1,y:0},
                    Point{x:2,y:1} => Point{x:1,y:1},

                    Point{x:-2,y:-1} => Point{x:-1,y:-1},
                    Point{x:-2,y:0} => Point{x:-1,y:0},
                    Point{x:-2,y:1} => Point{x:-1,y:1},

                    // pure diagonal pull.  Only matters for knots > 2
                    Point{x:2,y:2} => Point{x:1,y:1},
                    Point{x:2,y:-2} => Point{x:1,y:-1},
                    Point{x:-2,y:2} => Point{x:-1,y:1},
                    Point{x:-2,y:-2} => Point{x:-1,y:-1},

                    // something large for the obvious error code
                    _ => error_point,
                };
                assert!(change != error_point);
                
                rope[k] = rope[k] + change;
            }

            tail_points.insert(rope[cknots-1]);
        }
    }
    println!("{}", tail_points.len());
}

fn main() {
    let part = std::env::args().nth(1).unwrap();

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}