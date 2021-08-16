use crate::positional::digits::to_digits;
use itertools::Itertools;

pub fn run() {
    let bound = upper_bound();
    let valid_digits: usize = (3..bound)
        .filter(|d| *d == digits_factorial_sum(&to_digits(*d)))
        .sum();

    println!("{:?}", valid_digits)
}

fn digits_factorial_sum(digits: &[usize]) -> usize {
    digits.iter().map(|&d| factorial(d)).sum()
}

fn upper_bound() -> usize {
    let mut n = 1;
    loop {
        let max_num = max_n_digit_num(n);
        let max_sum = max_n_digit_factorial_sum(n);
        if max_sum < max_num {
            break;
        }
        n += 1
    }
    max_n_digit_num(n)
}

fn max_n_digit_num(n: usize) -> usize {
    let max_digit = 9;
    (0..n).map(|_| max_digit).join("").parse().unwrap()
}

fn max_n_digit_factorial_sum(n: usize) -> usize {
    let max_digit = 9;
    (0..n).map(|_| factorial(max_digit)).sum()
}

fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::positional::digits::to_digits;

    #[test]
    fn test_upper_bound_returns_proper_bound() {
        // when
        let result = upper_bound();

        // then
        let expected = 9999999;
        assert_eq!(expected, result)
    }

    #[test]
    fn test_max_n_digit_returns_num_with_all_9() {
        // given
        let n = 5;

        // when
        let result = max_n_digit_num(n);

        // then
        let expected = 99999;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_digits_factorial_sum_returns_proper_sum_for_number_145() {
        // given
        let num: usize = 145;

        // when
        let result = digits_factorial_sum(&to_digits(num));

        // then
        let expected = 145;
        assert_eq!(expected, result);
    }
}
