#![feature(test)]
#![feature(int_log)]

use std::env;

mod combinatorics;
mod common;
mod positional;
mod primes;
mod problems;
mod words;

use crate::common::data::{read_to_str, read_to_vec};

fn main() {
    use std::time::Instant;
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Usage: cargo run <problem_num>");
    }

    let problem_num: u16 = argv[1].parse().expect("Problem num must be a number");
    let now = Instant::now();

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
        29 => problems::p29_distinct_powers::run(),
        30 => problems::p30_digit_fifth_powers::run(),
        32 => problems::p32_pandigital_products::run(),
        31 => problems::p31_coin_sums::run(),
        33 => problems::p33_digit_cancelling_fractions::run(),
        34 => problems::p34_digit_factorials::run(),
        35 => problems::p35_circular_primes::run(),
        36 => problems::p36_double_base_palindromes::run(),
        37 => problems::p37_truncatable_primes::run(),
        38 => problems::p38_pandigital_multiples::run(),
        39 => problems::p39_integer_right_triangles::run(),
        40 => problems::p40_champernownes_constant::run(),
        41 => problems::p41_pandigital_prime::run(),
        42 => problems::p42_coded_triangle_numbers::run(&read_to_str(problem_num)),
        43 => problems::p43_sub_string_divisibility::run(),
        44 => problems::p44_pentagon_numbers::run(),
        45 => problems::p45_triangular_pentagonal_and_hexagonal::run(),
        46 => problems::p46_goldbachs_other_conjecture::run(),
        47 => problems::p47_distinct_primes_factors::run(),
        49 => problems::p49_prime_permutations::run(),
        50 => problems::p50_consecutive_prime_sum::run(),
        51 => problems::p51_prime_digit_replacements::run(),
        52 => problems::p52_permuted_multiples::run(),
        53 => problems::p53_combinatoric_selections::run(),
        54 => problems::p54_poker_hands::run(&read_to_vec(problem_num)),
        75 => problems::p75_singular_integer_right_triangles::run(),
        _ => panic!("Problem num: {} not implemented", problem_num),
    }
    let elapsed = now.elapsed();
    println!("\nFinished in: {:.2?}", elapsed);
}
