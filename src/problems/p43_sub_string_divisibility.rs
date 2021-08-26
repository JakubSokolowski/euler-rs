use crate::positional::digits::{from_digits, to_digits};
use itertools::Itertools;

pub fn run() {
    let nums: Vec<usize> = (0..=9)
        .permutations(10)
        .map(|d| from_digits(&d))
        .filter(|d| is_sub_str_divisible(*d))
        .collect();

    let sum: usize = nums.iter().sum();

    println!("Nums: {:?}", nums);
    println!("Sum of divisible numbers: {}", sum);
}

pub fn is_sub_str_divisible(num: usize) -> bool {
    let digits = to_digits(num);
    if digits.len() != 10 {
        return false;
    }
    let divisors = vec![2, 3, 5, 7, 11, 13, 17];
    digits
        .windows(3)
        .skip(1)
        .zip(divisors.iter())
        .map(|(d, div)| from_digits(d) % div == 0)
        .all(|d| d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sub_str_divisible_returns_true_when_each_subsequent_window_is_divisible_by_divisor()
    {
        // given
        let num = 1406357289;

        // when
        let result = is_sub_str_divisible(num);

        // then
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_sub_str_divisible_returns_false_when_some_window_is_not_divisible_by_divisor() {
        // given
        let num = 1406357298;

        // when
        let result = is_sub_str_divisible(num);

        // then
        let expected = false;
        assert_eq!(result, expected);
    }
}
