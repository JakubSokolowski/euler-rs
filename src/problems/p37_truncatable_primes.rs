use crate::positional::digits::{from_digits, to_digits};
use crate::primes::sieve::prime_lookup;
use std::collections::HashSet;

pub fn run() {
    let bound = 1_000_000;
    let lookup = prime_lookup(bound);
    let truncatable: Vec<&usize> = lookup
        .iter()
        .filter(|p| **p > 7 && is_truncatable_prime(**p, &lookup))
        .collect();

    let sum: usize = truncatable.iter().cloned().sum();

    println!("{} {:?} {}", truncatable.len(), truncatable, sum);
}

pub fn is_truncatable_prime(num: usize, lookup: &HashSet<usize>) -> bool {
    let as_digits = to_digits(num);
    let num_digits = as_digits.len();

    for idx in 1..num_digits {
        let ltr = from_digits(&as_digits[..idx]);
        let rtl = from_digits(&as_digits[idx..]);
        if !lookup.contains(&ltr) || !lookup.contains(&rtl) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_truncatable_prime_returns_true_for_3797() {
        // given
        let num = 3797;
        let lookup = prime_lookup(5000);

        // when
        let result = is_truncatable_prime(num, &lookup);

        // then
        let expected = true;
        assert_eq!(expected, result);
    }
}
