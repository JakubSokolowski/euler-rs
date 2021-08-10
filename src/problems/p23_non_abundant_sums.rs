use std::collections::HashSet;

use itertools::Itertools;

use crate::primes::factors;

const ABUNDANT_MAX: usize = 28123;

pub fn run() {
    println!("{:?}", factors::factors(12));

    let all_abundant: Vec<usize> = (1..ABUNDANT_MAX).filter(|&n| is_abundant(n)).collect();

    println!("{}", all_abundant.len());

    let abundant_sums: HashSet<usize> = all_abundant
        .iter()
        .cartesian_product(all_abundant.iter())
        .map(|(x, y)| x + y)
        .collect();

    let all_that_cannot: usize = (1..ABUNDANT_MAX)
        .filter(|&n| !abundant_sums.contains(&n))
        .sum();

    println!("{}", all_that_cannot);
}

pub fn factor_sum(num: usize) -> usize {
    factors::factors(num).iter().sum()
}

pub fn is_abundant(num: usize) -> bool {
    factor_sum(num) > num
}
