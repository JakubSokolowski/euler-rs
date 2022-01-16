use crate::words::alphabet::is_palindrome;
use num::bigint::ToBigInt;
use num::BigInt;

pub fn run() {
    let bound = 10_000;
    let max_iter = 50;
    let count = count_lychrel(bound, max_iter);
    println!("There are {} Lychrel numbers below {}", count, bound);
}

pub fn count_lychrel(bound: usize, max_iter: usize) -> usize {
    (0..bound)
        .filter(|n| is_lychrel(n.to_bigint().unwrap(), max_iter))
        .count()
}

pub fn is_lychrel(num: BigInt, max_iter: usize) -> bool {
    let mut curr = num;
    for _ in 0..max_iter {
        curr += rev(&curr);
        if is_palindrome(&curr.to_string()) {
            return false;
        }
    }
    true
}

pub fn rev(num: &BigInt) -> BigInt {
    num.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::ToBigInt;

    #[test]
    fn test_is_not_lychrel() {
        assert_eq!(is_lychrel(349.to_bigint().unwrap(), 50), false);
        assert_eq!(is_lychrel(47.to_bigint().unwrap(), 50), false);
    }

    #[test]
    fn test_is_lychrel() {
        assert_eq!(is_lychrel(196.to_bigint().unwrap(), 50), true);
        assert_eq!(is_lychrel(10677.to_bigint().unwrap(), 50), true);
    }
}
