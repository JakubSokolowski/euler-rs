use std::collections::HashSet;

use itertools::Itertools;

use crate::positional::digits::{from_digits, to_digits};
use crate::primes::sieve::{e_sieve, prime_lookup};

pub fn run() {
    let primes = four_digit_primes();
    let lookup: HashSet<usize> = prime_lookup(9999);

    for prime in primes {
        match prime_permutations(prime, &lookup) {
            None => {}
            Some(p) => match find_sequence(&p) {
                None => {}
                Some(d) => {
                    let joined_num = d.iter().map(|s| s.to_string()).join("");
                    println!(
                        "Found permutations: {:?} With sequence: {:?} {}",
                        p, d, joined_num
                    );
                }
            },
        }
    }
}

fn find_sequence(nums: &[usize]) -> Option<Vec<usize>> {
    nums.iter()
        .cloned()
        .permutations(3)
        .find(|p| is_same_difference_sequence(p))
}

fn is_same_difference_sequence(nums: &[usize]) -> bool {
    nums.windows(2).map(|w| w[1] - w[0]).unique().count() == 1
}

fn four_digit_primes() -> Vec<usize> {
    e_sieve(10000).into_iter().filter(|&n| n >= 1000).collect()
}

fn prime_permutations(prime: usize, primes: &HashSet<usize>) -> Option<Vec<usize>> {
    let permutations: Vec<usize> = to_digits(prime)
        .into_iter()
        .permutations(4)
        .map(|g| from_digits(&g))
        .filter(|d| primes.contains(d))
        .unique()
        .sorted()
        .collect();

    if permutations.len() >= 3 {
        let every_perm_4_digit = permutations.iter().all(|p| to_digits(*p).len() == 4);

        if every_perm_4_digit {
            return Some(permutations);
        }
    }

    None
}
