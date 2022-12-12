#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn part1() {
}

fn part2() {
}

fn main() {
    let part = std::env::args().nth(1).unwrap();

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}