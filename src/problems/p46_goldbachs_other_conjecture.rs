use crate::primes::is_prime::is_prime;
use crate::primes::sieve::prime_lookup;
use num_integer::Roots;
use std::collections::HashSet;

pub fn run() {
    let bound = 1_000_000;

    let primes = prime_lookup(bound);
    let composites = odd_composites(bound);

    for num in composites {
        match check_conjecture(num, &primes) {
            None => {
                println!("Conjecture does not hold for num: {}", num);
                break;
            }
            Some(_) => {}
        }
    }
}

fn odd_composites(bound: usize) -> Vec<usize> {
    let mut num = 3;
    let mut composites: Vec<usize> = Vec::with_capacity(bound);
    while num <= bound - 2 {
        num += 2;
        if !is_prime(num as i32) {
            composites.push(num)
        }
    }
    composites
}

fn check_conjecture(composite: usize, prime_lookup: &HashSet<usize>) -> Option<(usize, usize)> {
    let square_bound = composite.sqrt();

    for square in 1..=square_bound {
        let rest = composite - 2 * square.pow(2);
        if prime_lookup.contains(&rest) {
            return Some((rest, square));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes::sieve::prime_lookup;

    #[test]
    fn odd_composites_returns_vec_with_odd_composite_numbers_lesser_eq_to_bound() {
        // given
        let bound = 25;

        // when
        let composites = odd_composites(bound);

        // then
        let expected: Vec<usize> = vec![9, 15, 21, 25];
        assert_eq!(expected, composites);
    }

    #[test]
    fn check_conjecture_returns_true_for_27() {
        // given
        let num = 27;
        let primes = prime_lookup(num);

        // when
        let result = check_conjecture(num, &primes);

        // then
        let expected = Some((19, 2));
        assert_eq!(result, expected);
    }
}
