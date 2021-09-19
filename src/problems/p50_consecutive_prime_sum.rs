use std::collections::HashSet;

use itertools::Itertools;

use crate::primes::sieve::prime_lookup;

pub fn run() {
    let bound = 1_000_000;
    let result = largest_consecutive(bound);
    println!("Largest consecutive prime sum: {:?}", result);
}

fn largest_consecutive(bound: usize) -> (usize, usize) {
    let lookup = prime_lookup(bound);
    let ordered: Vec<usize> = lookup.clone().into_iter().sorted().collect();

    let mut largest = (0, 0);

    for group_size in 2..ordered.len() {
        let sum = first_consecutive_sum(&ordered, &lookup, group_size);
        match sum {
            None => {}
            Some(x) => {
                if largest.0 < group_size {
                    let (prime, latest) = x;
                    largest = (group_size, prime);
                    if prime + latest + 1 >= bound {
                        break;
                    }
                }
            }
        }
    }

    largest
}

fn first_consecutive_sum(
    ordered_primes: &[usize],
    primes_lookup: &HashSet<usize>,
    group_size: usize,
) -> Option<(usize, usize)> {
    for group in ordered_primes.windows(group_size) {
        let sum = &group.iter().copied().sum();
        if primes_lookup.contains(sum) {
            return Some((*sum, *group.last().unwrap()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_consecutive_sum_returns_sum_if_such_exists() {
        // given
        let bound = 100;
        let lookup = prime_lookup(bound);
        let ordered: Vec<usize> = lookup.clone().into_iter().sorted().collect();
        let group_size = 6;

        // when
        let result = first_consecutive_sum(&ordered, &lookup, group_size);

        // then
        let expected = Some((41, 13));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_largest_consecutive_returns_953_for_bound_1000() {
        // given
        let bound = 1000;

        // when
        let result = largest_consecutive(bound);

        // then
        let expected = (21, 953);
        assert_eq!(expected, result);
    }
}
