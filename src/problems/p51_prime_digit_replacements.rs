use std::collections::HashSet;

use itertools::Itertools;

use crate::positional::digits::{num_of_digits, to_digits};
use crate::primes::sieve::prime_lookup;

pub fn run() {
    let bound = 1_000_000;
    let desired_count = 8;
    let prime = smallest_prime(bound, desired_count);
    println!(
        "Smallest replacement prime of count {} is {}",
        desired_count, prime
    );
}

fn smallest_prime(bound: usize, desired_count: usize) -> usize {
    let primes = prime_lookup(bound);

    for prime in primes.iter().sorted() {
        for d in to_digits(*prime).iter().unique() {
            let count = count_prime_replacements(*prime, *d, &primes);
            if count == desired_count {
                return *prime;
            }
        }
    }

    unreachable!("Prime bound reached")
}

fn replace_digit(prime: usize, index: usize, digit: usize) -> usize {
    let num_digits = num_of_digits(prime);

    assert!(index < num_digits, "Digit index out of bounds");

    let curr_digit = nth_digit(prime, index);
    let num_digits_to_end = num_digits - 1 - index;
    let to_sub = curr_digit * 10_usize.pow(num_digits_to_end as u32);
    let to_add = digit * 10_usize.pow(num_digits_to_end as u32);
    prime - to_sub + to_add
}

fn nth_digit(num: usize, n: usize) -> usize {
    num / 10_usize.pow((num_of_digits(num) - 1 - n) as u32) % 10
}

fn count_prime_replacements(
    prime: usize,
    digit_to_replace: usize,
    lookup: &HashSet<usize>,
) -> usize {
    let prime_digits = to_digits(prime);

    let positions: Vec<_> = prime_digits
        .iter()
        .enumerate()
        .filter_map(|(pos, d)| {
            if *d == digit_to_replace {
                Some(pos)
            } else {
                None
            }
        })
        .collect();

    1 + (0..=9)
        .filter_map(|d| {
            if d == digit_to_replace || d == 0 && positions[0] == 0 {
                return None;
            }

            let num = replace_digits(prime, &positions, d);

            if lookup.contains(&num) {
                Some(num)
            } else {
                None
            }
        })
        .count()
}

fn replace_digits(prime: usize, indices: &[usize], digit: usize) -> usize {
    indices
        .iter()
        .fold(prime, |acc, pos| replace_digit(acc, *pos, digit))
}

#[cfg(test)]
mod tests {
    use crate::primes::sieve::prime_lookup;

    use super::*;

    #[test]
    fn test_nth_digit_returns_nth_digit_of_num() {
        let num = 56413;
        assert_eq!(nth_digit(num, 0), 5);
        assert_eq!(nth_digit(num, 1), 6);
        assert_eq!(nth_digit(num, 2), 4);
        assert_eq!(nth_digit(num, 3), 1);
        assert_eq!(nth_digit(num, 4), 3);
    }

    #[test]
    fn test_replace_digit_returns_num_with_index_set_to_specified_digit() {
        assert_eq!(replace_digit(54321, 0, 9), 94321);
        assert_eq!(replace_digit(54321, 1, 9), 59321);
        assert_eq!(replace_digit(54321, 2, 9), 54921);
        assert_eq!(replace_digit(54321, 3, 9), 54391);
        assert_eq!(replace_digit(54321, 4, 9), 54329);
    }

    #[test]
    fn test_count_prime_replacements_returns_proper_count_for_2_digit_prime() {
        // given
        let prime = 13;
        let lookup = prime_lookup(100);
        let digit = 1;

        // when
        let count = count_prime_replacements(prime, digit, &lookup);

        // then
        assert_eq!(count, 6);
    }

    #[test]
    fn test_count_prime_replacements_returns_proper_count_for_5_digit_prime() {
        // given
        let prime = 56003;
        let lookup = prime_lookup(100000);
        let digit = 0;

        // when
        let count = count_prime_replacements(prime, digit, &lookup);

        // then
        assert_eq!(count, 7);
    }

    #[test]
    fn test_smallest_prime_returns_13_for_desired_count_4() {
        // when
        let prime = smallest_prime(100, 6);

        // then
        assert_eq!(prime, 13);
    }
}
