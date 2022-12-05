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

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let sep = lines.iter().position(|s| s == "" ).unwrap();
    let buckets = &lines[0..sep-1];
    let instructions = &lines[sep+1..];

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
    /*
    for stack in stacks {
        println!("{}", stack.join(" "));
        println!("-----");
    }
    */

    for i in instructions {
        println!("-----");
        println!("{}", i);
        let parts = i.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from_bucket = parts[3].parse::<usize>().unwrap() - 1;
        let to_bucket = parts[5].parse::<usize>().unwrap() - 1;

        for c in 0..count {
            let e = stacks[from_bucket].pop().unwrap();
            stacks[to_bucket].push(e);
        }

        for stack in &stacks {
            println!("{}", stack.join(" "));
        }
    }

    println!("=====");
    for mut stack in stacks {
        println!("{}", stack.pop().unwrap());
    }
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let sep = lines.iter().position(|s| s == "" ).unwrap();
    let buckets = &lines[0..sep-1];
    let instructions = &lines[sep+1..];

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
    /*
    for stack in stacks {
        println!("{}", stack.join(" "));
        println!("-----");
    }
    */

    for i in instructions {
        println!("-----");
        println!("{}", i);
        let parts = i.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from_bucket = parts[3].parse::<usize>().unwrap() - 1;
        let to_bucket = parts[5].parse::<usize>().unwrap() - 1;

        let mut temp : Vec<String> = vec![];
        for c in 0..count {
            let e = stacks[from_bucket].pop().unwrap();
            temp.push(e);
        }
        temp.reverse();
        for e in temp {
            stacks[to_bucket].push(e);
        }

        for stack in &stacks {
            println!("{}", stack.join(" "));
        }
    }


    println!("=====");
    for mut stack in stacks {
        println!("{}", stack.pop().unwrap());
    }
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}