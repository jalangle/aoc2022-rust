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

    let mut elf_calories = vec![];

    for elf in lines.split(|element| element.len() == 0) {
        let calories = elf.iter().map(|s| s.parse::<i32>().unwrap() ).sum::<i32>();
        elf_calories.push(calories);
    }

    let max_calories = elf_calories.iter().fold(std::i32::MIN, |a,b| a.max(*b));
    println!("{}", max_calories.to_string());
}

fn part2(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);

    let mut elf_calories = vec![];

    for elf in lines.split(|element| element.len() == 0) {
        let calories = elf.iter().map(|s| s.parse::<i32>().unwrap() ).sum::<i32>();
        elf_calories.push(calories);
    }

    elf_calories.sort();
    elf_calories.reverse();
    let top_three = elf_calories[0..3].iter().sum::<i32>();
    println!("{}", top_three.to_string());
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}