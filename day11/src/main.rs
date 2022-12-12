#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::VecDeque;

#[path = "inputdata1.rs"] mod input1data;
use input1data::input1data;

#[path = "testdata1.rs"] mod test1data;
use test1data::test1data;

fn part1() {
    let mut monkeys = input1data();
    for m in 0..monkeys.len() {
        println!("{:?}", monkeys[m]);
    }

    for round in 1..21 {

        for m in 0..monkeys.len() {
            // do inspections
            monkeys[m].items = monkeys[m].items.iter().map(|x| (monkeys[m].operation)(*x)).collect();
            // keep track of number of inspections 
            monkeys[m].inspections += monkeys[m].items.len();
            // my relief
            monkeys[m].items = monkeys[m].items.iter().map(|x| x / 3).collect();

            let mut true_items : VecDeque<u64> = VecDeque::new();
            let mut false_items : VecDeque<u64> = VecDeque::new();

            for _ in 0..monkeys[m].items.len() {
                let monkey : &mut _ = &mut monkeys[m];
                let item = monkey.items.pop_front().unwrap();
                let x : u64 = item % monkey.divisible_by;
                if x == 0 {
                    true_items.push_back(item);
                }
                else {
                    false_items.push_back(item);
                }
            }
            let id_to = monkeys[m].to_true;
            monkeys[id_to].items.append(&mut true_items);

            let id_false = monkeys[m].to_false;
            monkeys[id_false].items.append(&mut false_items);

        }
/*
        println!("--------  ROUND {} ------------------", round);
        for m in 0..monkeys.len() {
            println!("{:?}", monkeys[m]);
        }
*/
    }

    println!("--------  SORTED ------------------");
    monkeys.sort_by(|a,b| b.inspections.cmp(&a.inspections));
    for m in 0..monkeys.len() {
        println!("{:?}", monkeys[m]);
    }
    println!("{}", monkeys[0].inspections * monkeys[1].inspections);
}

fn truncate(cm: &u64, i: &u64) -> u64 {
    if i > cm {
        return i % cm;
    }
    return *i
}

fn part2() {
    let mut monkeys = test1data();
        for m in 0..monkeys.len() {
            println!("{:?}", monkeys[m]);
        }

    let mut common_mult = 1;
    for m in &monkeys {
        common_mult *= m.divisible_by;
    }


    for round in 1..10001 {
        //println!("--------  ROUND {} ------------------", round);

        for m in 0..monkeys.len() {
            //println!("{:?}", monkeys[m]);
            // do inspections
            monkeys[m].items = monkeys[m].items.iter().map(|x| (monkeys[m].operation)(*x)).collect();
            // keep track of number of inspections
            //println!("INSPECTED: {}", monkeys[m].items.len());
            monkeys[m].inspections += monkeys[m].items.len();
            // my relief
            monkeys[m].items = monkeys[m].items.iter().map(|x| return truncate(&common_mult, x) ).collect();

            let mut true_items : VecDeque<u64> = VecDeque::new();
            let mut false_items : VecDeque<u64> = VecDeque::new();

            for _ in 0..monkeys[m].items.len() {
                let monkey : &mut _ = &mut monkeys[m];
                let item = monkey.items.pop_front().unwrap();
                let x : u64 = item % monkey.divisible_by;
                if x == 0 {
                    true_items.push_back(item);
                }
                else {
                    false_items.push_back(item);
                }
            }
            let id_to = monkeys[m].to_true;
            monkeys[id_to].items.append(&mut true_items);

            let id_false = monkeys[m].to_false;
            monkeys[id_false].items.append(&mut false_items);

        }
/*
        println!("--------  ROUND {} ------------------", round);
        for m in 0..monkeys.len() {
            println!("{:?}", monkeys[m]);
        }
*/
    }

    println!("--------  SORTED ------------------");
    monkeys.sort_by(|a,b| b.inspections.cmp(&a.inspections));
    for m in 0..monkeys.len() {
        println!("{:?}", monkeys[m]);
    }
    println!("{}", monkeys[0].inspections * monkeys[1].inspections);

}

fn main() {
    let part = std::env::args().nth(1).unwrap();

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}