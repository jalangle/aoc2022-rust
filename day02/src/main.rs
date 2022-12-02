#![allow(dead_code)]

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

fn score_choice(c: char) -> i32 {
    match c {
        'A' => 1 , // Rock,
        'B' => 2 , // Paper,
        'C' => 3 , // Scisors,
        _ => -1,
    }
}

fn score(them: char, you: char) -> i32 {
    if you == 'A' && them == 'C' {
        return 6 + score_choice(you)
    }
    else if you == 'A' && them == 'A' {
        return 3 + score_choice(you)
    }

    else if you == 'B' && them == 'A' {
        return 6 + score_choice(you)
    }
    else if you == 'B' && them == 'B' {
        return 3 + score_choice(you)
    }

    else if you == 'C' && them == 'B' {
        return 6 + score_choice(you)
    }
    else if you == 'C' && them == 'C' {
        return 3 + score_choice(you)
    }

    return score_choice(you)
}

fn translate(c: char) -> char {
    return match c {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => '0', // shut up, rust
    }
}

fn score_line1(line: &String) -> i32 {
    // a draw
    let them = line.chars().nth(0).unwrap();
    let you = translate(line.chars().nth(2).unwrap());
    return score(them, you)
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
    let score = lines.iter().map(|l| score_line1(l)).sum::<i32>();
    println!("{}", score);
}


fn translate2(t: char, s: char) -> char {
    if s == 'X' /* lose */{
        return match t {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => '0' // shut up, rust
        }        
    }
    else if s == 'Z' /* win */{
        return match t {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => '0' // shut up, rust
        }        
    }

    return t
}

fn score_line2(line: &String) -> i32 {
    // a draw
    let them = line.chars().nth(0).unwrap();
    let strategy = line.chars().nth(2).unwrap();
    let you = translate2(them, strategy);
    return score(them, you)
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
    let score = lines.iter().map(|l| score_line2(l)).sum::<i32>();
    println!("{}", score);
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}