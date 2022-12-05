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

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace() );
}

fn lines_to_parts(lines: Vec<String>) -> (Vec<Vec<String>>, Vec<String>) {
    let sep = lines.iter().position(|s| s == "" ).unwrap();
    let buckets = &lines[0..sep-1];
    let instructions : Vec<String> = (&lines[sep+1..]).to_vec();

    let mut stacks : Vec<Vec<String>> = vec![];
    let count_of_buckets = buckets[0].chars().collect::<Vec<char>>().chunks(4).count();

    for i in 0..count_of_buckets {
        stacks.push(Vec::<String>::new());
    }

    for x in buckets {
        let mut index : usize = 0;

        for y in x.chars().collect::<Vec<char>>().chunks(4) {
            let mut s = String::from_iter(y);
            remove_whitespace(&mut s);
            if s != "" {
                stacks[index].insert(0, s);
            }
            index+=1;
        }
    }

    return (stacks, instructions);
}

fn parse_instruction(i: String) -> (usize, usize, usize) {
        let parts = i.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from_bucket = parts[3].parse::<usize>().unwrap() - 1;
        let to_bucket = parts[5].parse::<usize>().unwrap() - 1;

        return (count, from_bucket, to_bucket);
}

fn solution_str(stacks: Vec<Vec<String>>) -> String {
    return stacks.iter().map(|stack| return stack.last().unwrap().clone() ).collect::<Vec<String>>().join(" ");
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let (mut stacks, instructions) = lines_to_parts(lines);

    for i in instructions {
        let (count, from_bucket, to_bucket) = parse_instruction(i);

        for c in 0..count {
            let e = stacks[from_bucket].pop().unwrap();
            stacks[to_bucket].push(e);
        }
    }

    println!("{}", solution_str(stacks));
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let (mut stacks, instructions) = lines_to_parts(lines);

    for i in instructions {
        let (count, from_bucket, to_bucket) = parse_instruction(i);

        let mut temp : Vec<String> = vec![];
        for c in 0..count {
            let e = stacks[from_bucket].pop().unwrap();
            temp.push(e);
        }
        temp.reverse();
        for e in temp {
            stacks[to_bucket].push(e);
        }
    }

    println!("{}", solution_str(stacks));
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part1(&file),
        None => println!("No file"),
    }
}