use crate::positional::digits::to_digits;
use std::collections::HashSet;

pub fn run() {
    let bound = 1_000_000;
    let smallest = find_smallest(bound);
    println!("Smallest permuted multiple is: {}", smallest);
}

pub fn find_smallest(bound: usize) -> usize {
    for i in 1..=bound {
        if is_permuted_multiple(i) {
            return i;
        }
    }
    unreachable!("Bound reached")
}

pub fn is_permuted_multiple(num: usize) -> bool {
    let digits = to_digits(num);
    let uniq_digits: HashSet<_> = digits.iter().collect();
    for m in [2, 3, 4, 5, 6].iter() {
        let mult_digits = to_digits(num * m);
        let uniq_mult: HashSet<_> = mult_digits.iter().collect();
        if uniq_mult != uniq_digits {
            return false;
        }
    }
    true
}
