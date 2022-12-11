#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn file_to_lines() -> Vec<String> {
    let bytes = include_bytes!("input1.txt");
    let lines = String::from_utf8_lossy(bytes)
    .split("\n")
    .collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
    return lines
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Instruction {
    name: String,
    ticks: usize,
    value: Option<i64>,
}

fn line_to_struct(line: &String) -> Instruction {
    let parts : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
    let mut i = Instruction { name: parts[0].clone(), ticks: 1, value: None };
    if i.name == "noop" {
        i.ticks = 1;
    }
    else if i.name == "addx" {
        i.ticks = 2;
        i.value = Some(parts[1].parse::<i64>().unwrap());
    }
    return i
}
fn part1() {
    let instructions : Vec<Instruction> = file_to_lines().iter().map(|l| line_to_struct(l)).collect();

    let mut total_strength = 0;
    for tick in (0..6).map(|x| return 20 + x * 40 ) {
        let mut elapsed_ticks = 0;
        let mut current_value : i64 = 1;
        for index in 0..instructions.len() {
            elapsed_ticks += instructions[index].ticks;
            if elapsed_ticks >= tick {
                break;
            }
            current_value += instructions[index].value.unwrap_or(0);
        }
        println!("{:?} {:?}", tick, current_value);
        let strength = tick as i64 * current_value;
        total_strength += strength;

    }

    println!("------");
    println!("{}", total_strength); //15080 too high
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