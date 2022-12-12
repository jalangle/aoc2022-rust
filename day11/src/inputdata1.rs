use std::collections::VecDeque;

#[path = "monkey.rs"] mod monkey;
use monkey::Monkey;

pub fn input1data() -> Vec<Monkey> {
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
