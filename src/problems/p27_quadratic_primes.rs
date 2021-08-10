extern crate test;

use itertools::Itertools;

use crate::primes::is_prime;

pub fn run() {
    find_max_coefficient_product(999, 1000);
}

fn find_max_coefficient_product(a_abs_limit: i32, b_abs_limit: i32) -> i32 {
    println!(
        "Searching for quadratic primes a: {} b: {}",
        a_abs_limit, b_abs_limit
    );

    let mut attempts = 0;
    let res = (-a_abs_limit..a_abs_limit)
        .cartesian_product(-b_abs_limit..b_abs_limit)
        .inspect(|_| attempts += 1)
        .map(|(a, b)| ((a, b), num_generated_primes(a, b)))
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    let (a, b) = res.0;
    let product = a * b;

    println!(
        "Attempts: {} Max consecutive primes: {:?} Coefficient product: {}",
        attempts, res, product
    );

    product
}

fn num_generated_primes(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        let maybe_prime = quadratic_formula(a, b, n);
        if is_prime::is_prime(maybe_prime.abs()) {
            n += 1;
        } else {
            break;
        }
    }
    n
}

fn quadratic_formula(a: i32, b: i32, n: i32) -> i32 {
    n.pow(2) + a * n + b
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_num_generated_primes_for_a79_b1601() {
        // given
        let a = -79;
        let b = 1601;

        // when
        let result = num_generated_primes(a, b);

        // then
        let expected = 80;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_num_generated_primes_for_a1_b41() {
        // given
        let a = 1;
        let b = 41;

        // when
        let result = num_generated_primes(a, b);

        // then
        let expected = 40;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_num_generated_primes_for_a999_b61() {
        // given
        let a = -999;
        let b = 61;

        // when
        let result = num_generated_primes(a, b);

        // then
        let expected = 11;
        assert_eq!(expected, result);
    }

    #[bench]
    fn bench_find_coefficient(bencher: &mut Bencher) {
        let a = 999;
        let b = 1000;
        bencher.iter(|| find_max_coefficient_product(a, b));
    }
}
