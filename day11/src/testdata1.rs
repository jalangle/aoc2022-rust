use std::collections::VecDeque;
#[path = "monkey.rs"] mod monkey;
use monkey::Monkey;

pub fn test1data() -> Vec<Monkey> {
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
