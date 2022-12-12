use std::collections::VecDeque;

pub type Operation = fn(u64) -> u64;

#[derive(Debug)]
pub struct Monkey {
    pub id: i32,
    pub items : VecDeque<u64>,

    pub operation : Operation,

    /* test  */
    pub divisible_by : u64,
    pub to_true : usize,
    pub to_false : usize,

    pub inspections : usize,
}
