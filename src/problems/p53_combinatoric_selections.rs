use itertools::Itertools;
use num::bigint::ToBigInt;

use crate::combinatorics::binomial::binomial;

pub fn run() {
    let bound = 100;
    let count = count_greater(bound);
    println!("Found: {} greater", count);
}

pub fn count_greater(bound: u64) -> usize {
    (1..=bound)
        .cartesian_product(1..=bound)
        .filter(|&(n, k)| binomial(n, k) > 1_000_000.to_bigint().unwrap())
        .count()
}
