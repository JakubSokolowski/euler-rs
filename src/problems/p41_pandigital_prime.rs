use crate::positional::digits::from_digits;
use crate::primes::is_prime::is_prime;
use itertools::Itertools;

pub fn run() {
    let result = largest_n_pandigital_prime();
    println!("Largest pandigital: {}", result);
}

pub fn largest_n_pandigital_prime() -> usize {
    (1..=9)
        .map(|d| largest_pandigital_prime(d).unwrap_or(1))
        .max()
        .unwrap()
}

pub fn largest_pandigital_prime(num_digits: usize) -> Option<usize> {
    (1..=num_digits)
        .permutations(num_digits)
        .map(|d| from_digits(&d))
        .filter(|d| is_prime(*d as i32))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_n_pandigital_returns_largest_prime_with_given_num_of_digits() {
        // given
        let n = 4;

        // when
        let result = largest_pandigital_prime(n).unwrap();

        // then
        let expected = 4231;
        assert_eq!(result, expected);
    }
}
