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

    let lines : Vec<String> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
    lines
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let mut cwd : String = "".to_string();
    let mut current_command : String = "".to_string();

    let mut dirs : Vec<String> = vec![];
    let mut sizes : Vec<String> = vec![];

    for l in lines {
        if l.starts_with("$") {
            let command_parts : Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
            current_command = command_parts[1].clone();
            if command_parts[1] == "cd" {
                if command_parts[2] == "/" {
                    cwd = "".to_string();
                }
                else if command_parts[2] == ".." {
                    let mut path_parts : Vec<String> = cwd.split('/').map(|s| s.to_string()).collect();
                    path_parts.pop();
                    cwd = path_parts.join("/");
                }
                else {
                    cwd += "/";
                    cwd += &command_parts[2];

                    if cwd != "" && !dirs.contains(&cwd) {
                        dirs.push(cwd.clone());
                    }
                }
                //println!("{} >", cwd);
            }
            else if command_parts[1] == "ls" {
                //println!("ls");
            }
        }
        else {
            if current_command == "ls" {
                if l.starts_with("dir") {

                }
                else {
                    let line_parts : Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
                    let size : usize = line_parts[0].parse::<usize>().unwrap();
                    let name = line_parts[1].to_string();
                    //println!("{} {}", size, name);
                    sizes.push(format!("{}-{}-{}", cwd, name, size));
                }
            }

        }
    }

    let mut total_size : usize = 0;
    for d in dirs {
        let mut objects_in_dir : Vec<String> = sizes.clone();
        objects_in_dir.retain(|s| s.starts_with(&d));
        let mut size : usize = 0;
        for o in objects_in_dir {
            let line_parts : Vec<String> = o.split('-').map(|s| s.to_string()).collect();
            size += line_parts[2].parse::<usize>().unwrap();
        }
        if size <= 100000 {
            total_size += size;
        }
    }
    println!("{}", total_size.to_string()); // 1331842 too low
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let mut cwd : String = "".to_string();
    let mut current_command : String = "".to_string();

    let mut dirs : Vec<String> = vec![];
    let mut sizes : Vec<String> = vec![];

    for l in lines {
        if l.starts_with("$") {
            let command_parts : Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
            current_command = command_parts[1].clone();
            if command_parts[1] == "cd" {
                if command_parts[2] == "/" {
                    cwd = "".to_string();
                }
                else if command_parts[2] == ".." {
                    let mut path_parts : Vec<String> = cwd.split('/').map(|s| s.to_string()).collect();
                    path_parts.pop();
                    cwd = path_parts.join("/");
                }
                else {
                    cwd += "/";
                    cwd += &command_parts[2];

                    if cwd != "" && !dirs.contains(&cwd) {
                        dirs.push(cwd.clone());
                    }
                }
                //println!("{} >", cwd);
            }
            else if command_parts[1] == "ls" {
                //println!("ls");
            }
        }
        else {
            if current_command == "ls" {
                if l.starts_with("dir") {

                }
                else {
                    let line_parts : Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
                    let size : usize = line_parts[0].parse::<usize>().unwrap();
                    let name = line_parts[1].to_string();
                    //println!("{} {}", size, name);
                    sizes.push(format!("{}-{}-{}", cwd, name, size));
                }
            }

        }
    }

    let max : usize = 70000000;
    let need : usize = 30000000;

    let mut total_size : usize = 0;
    for s in &sizes {
        let line_parts : Vec<String> = s.split('-').map(|s| s.to_string()).collect();
        total_size += line_parts[2].parse::<usize>().unwrap();
    }
    println!("HAVE: {}", total_size.to_string());

    let unused : usize = max - total_size;
    println!("UNUSED: {}", unused.to_string());

    let to_free = need-unused;

    let mut to_delete = max;

    for d in dirs {
        let mut objects_in_dir : Vec<String> = sizes.clone();
        objects_in_dir.retain(|s| s.starts_with(&d));
        let mut size : usize = 0;
        for o in objects_in_dir {
            let line_parts : Vec<String> = o.split('-').map(|s| s.to_string()).collect();
            size += line_parts[2].parse::<usize>().unwrap();
        }
        if size >= to_free  && size < to_delete {
            to_delete = size;
        }
    }
    println!("{}", to_delete); // 22685292 too high
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}