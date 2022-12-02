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
        'A' => 0 , // Rock,
        'B' => 0 , // Paper,
        'C' => 0 , // Scisors,
        'X' => 1 , // Rock,
        'Y' => 2 , // Paper,
        'Z' => 3 , // Scissors,
        _ => -1,
    }
}

fn score_line(line: &String) -> i32 {
    // a draw
    let them = line.chars().nth(0).unwrap();
    let you = line.chars().nth(2).unwrap();

    if you == 'X' && them == 'C' {
        return 6 + score_choice(you)
    }
    else if you == 'X' && them == 'A' {
        return 3 + score_choice(you)
    }

    else if you == 'Y' && them == 'A' {
        return 6 + score_choice(you)
    }
    else if you == 'Y' && them == 'B' {
        return 3 + score_choice(you)
    }

    else if you == 'Z' && them == 'B' {
        return 6 + score_choice(you)
    }
    else if you == 'Z' && them == 'C' {
        return 3 + score_choice(you)
    }

    return score_choice(you)
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
    let score = lines.iter().map(|l| score_line(l)).sum::<i32>();
    println!("{}", score);
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part1(&file),
        None => println!("No file"),
    }
}