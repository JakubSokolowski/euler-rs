use std::collections::HashSet;

use crate::positional::digits::{from_digits, to_digits};
use crate::primes::sieve::prime_lookup;

pub fn run() {
    let upper_bound = 999999;
    let count = count_circular_primes(upper_bound);

    println!("Found: {} circular primes", count);
}

pub fn count_circular_primes(bound: usize) -> usize {
    let primes = prime_lookup(bound);
    primes
        .iter()
        .filter_map(|&p| {
            if is_circular_prime(p, &primes) {
                Some(p)
            } else {
                None
            }
        })
        .count()
}

pub fn is_circular_prime(num: usize, prime_lookup: &HashSet<usize>) -> bool {
    get_rotations(num).iter().all(|r| prime_lookup.contains(r))
}

pub fn get_rotations(num: usize) -> Vec<usize> {
    let mut all_rotations: Vec<usize> = vec![num];
    let as_digits = to_digits(num);
    let num_rotations = as_digits.len() - 1;
    let mut prev_rot = as_digits;

    for _ in 0..num_rotations {
        prev_rot.rotate_left(1);
        let new_rot_num = from_digits(&prev_rot);
        all_rotations.push(new_rot_num)
    }

    all_rotations
}

#[cfg(test)]
mod tests {
    use crate::primes::sieve::prime_lookup;

    use super::*;

    #[test]
    fn test_get_rotations_returns_all_rotations_of_given_num() {
        // given
        let num = 197;

        // when
        let result = get_rotations(num);

        // then
        let expected: Vec<usize> = vec![197, 971, 719];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_rotations_returns_all_rotations_of_given_num_with_zero_at_some_position() {
        // given
        let num = 1907;

        // when
        let result = get_rotations(num);

        // then
        let expected: Vec<usize> = vec![1907, 9071, 719, 7190];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_circular_prime_returns_true_for_179() {
        // given
        let num = 197;
        let lookup = prime_lookup(1000);

        // when
        let result = is_circular_prime(num, &lookup);

        // then
        let expected = true;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_count_circular_primes_returns_proper_count_for_bound_100() {
        // given
        let bound = 100;

        // when
        let result = count_circular_primes(bound);

        // then
        let expected = 13;
        assert_eq!(expected, result);
    }
}
