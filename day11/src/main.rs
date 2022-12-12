#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::VecDeque;

type Operation = fn(u64) -> u64;

#[derive(Debug)]
struct Monkey {
    id: i32,
    items : VecDeque<u64>,

    operation : Operation,

    /* test  */
    divisible_by : u64,
    to_true : usize,
    to_false : usize,

    inspections : usize,
}

fn test1data() -> Vec<Monkey> {
    let data = vec![
    Monkey {
        id:0,
        items: VecDeque::from([79, 98]),
        operation: |x| x * 19,
        divisible_by: 23,
        to_true: 2,
        to_false: 3,
        inspections: 0,
    },
    Monkey {
      id:1,
      items: VecDeque::from([54, 65, 75, 74]),
      operation: |x| x + 6,
      divisible_by: 19,
      to_true: 2,
      to_false: 0,
      inspections: 0,
  },

    Monkey {
    id:2,
    items: VecDeque::from([79, 60, 97]),
    operation: |x| x * x,
    divisible_by: 13,
    to_true: 1,
    to_false: 3,
    inspections: 0,
    },

    Monkey {
        id:3,
        items: VecDeque::from([74]),
        operation: |x| x + 3,
        divisible_by: 17,
        to_true: 0,
        to_false: 1,
        inspections: 0,
    },
    ];
    return data
}

fn input1data() -> Vec<Monkey> {
    let data = vec![
    Monkey {
        id: 0,
        items: VecDeque::from([ 91, 58, 52, 69, 95, 54 ]),
        operation: |x| x * 13,
        divisible_by: 7,
        to_true: 1,
        to_false: 5,
        inspections: 0,
    },

    Monkey {
        id: 1,
        items: VecDeque::from([ 80, 80, 97, 84]),
        operation: |x| x * x,
        divisible_by: 3,
        to_true: 3,
        to_false: 5,
        inspections: 0,
    },

    Monkey {
        id: 2,
        items: VecDeque::from([ 86, 92, 71]),
        operation: |x| x + 7,
        divisible_by: 2,
        to_true: 0,
        to_false: 4,
         inspections: 0,
   },

    Monkey {
        id: 3,
        items: VecDeque::from([ 96, 90, 99, 76, 79, 85, 98, 61]),
        operation: |x| x + 4,
        divisible_by: 11,
        to_true: 7,
        to_false: 6,
        inspections: 0,
    },

    Monkey {
        id: 4,
        items: VecDeque::from([ 60, 83, 68, 64, 73]),
        operation: |x| x * 19,
        divisible_by: 17,
        to_true: 1,
        to_false: 0,
        inspections: 0,
    },

    Monkey {
        id: 5,
        items: VecDeque::from([ 96, 52, 52, 94, 76, 51, 57]),
        operation: |x| x + 3,
        divisible_by: 5,
        to_true: 7,
        to_false: 3,
        inspections: 0,
    },

    Monkey {
        id: 6,
        items: VecDeque::from([ 75]),
        operation: |x| x + 5,
        divisible_by: 13,
        to_true: 4,
        to_false: 2,
        inspections: 0,
    },

    Monkey {
        id: 7,
        items: VecDeque::from([ 83, 75]),
        operation: |x| x + 1,
        divisible_by: 19,
        to_true: 2,
        to_false: 6,
        inspections: 0,
    },

    ];
    return data
}

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
                let monkey : &mut Monkey = &mut monkeys[m];
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
    let mut monkeys = input1data();
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
                let monkey : &mut Monkey = &mut monkeys[m];
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