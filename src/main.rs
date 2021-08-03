#![feature(test)]

use std::env;

mod common;
mod problems;

use crate::common::data::{read_to_str, read_to_vec};

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Usage: cargo run <problem_num>");
    }

    let problem_num: u16 = argv[1].parse().expect("Problem num must be a number");

    match problem_num {
        11 => problems::p11_largest_product_in_a_grid::run(&read_to_vec(problem_num)),
        17 => problems::p17_number_letter_counts::run(),
        19 => problems::p19_counting_sundays::run(),
        22 => problems::p22_names_scores::run(&read_to_str(problem_num)),
        23 => problems::p23_non_abundant_sums::run(),
        24 => problems::p24_lexicographic_permutations::run(),
        26 => problems::p26_reciprocal_cycles::run(),
        27 => problems::p27_quadratic_primes::run(),
        28 => problems::p28_number_spiral_diagonals::run(),
        75 => problems::p75_singular_integer_right_triangles::run(),
        _ => panic!("Problem num: {} not implemented", problem_num),
    }
}
