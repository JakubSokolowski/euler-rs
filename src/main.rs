use std::env;

mod common;
mod problems;


use crate::common::data::{read_to_vec};

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Usage: cargo run <problem_num>");
    }

    let problem_num: u16 = argv[1].parse().expect("Problem num must be a number");

    match problem_num {
        11 => problems::p11_largest_product_in_a_grid::run(&read_to_vec(problem_num)),
        _ => panic!("Problem num: {} not implemented", problem_num)
    }
}
