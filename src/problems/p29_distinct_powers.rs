use itertools::Itertools;
use num_integer::Roots;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
struct AbPow {
    base: u32,
    exponent: u32,
}

impl AbPow {
    pub fn reduce(&mut self) {
        if let Some(r) = smallest_integer_root(self.base) {
            let (root, exponent) = r;
            self.base = root;
            self.exponent *= exponent;
        }
    }
}

pub fn run() {
    let lower = 2;
    let upper = 100;
    let distinct = distinct_powers(lower, upper);
    println!("{:?}", 100_i32.checked_pow(100));

    println!("Found: {} for a,b {}<= {}", distinct, lower, upper);
}

pub fn distinct_powers(lower: u32, upper: u32) -> usize {
    (lower..=upper)
        .cartesian_product(lower..=upper)
        .map(|(a, b)| {
            let mut pow = AbPow {
                base: a,
                exponent: b,
            };
            pow.reduce();
            pow
        })
        .unique()
        .count()
}

pub fn smallest_integer_root(num: u32) -> Option<(u32, u32)> {
    // a,b limit is 100, so for the smallest possible base (2)
    // the largest possible exponent would be 6
    // (2^6=64, 2^7=128 - outside of limit)
    // to find smallest integer root, check all nth possible roots
    // from
    let n_max = 6;
    let mut smallest = u32::MAX;
    let mut exponent: u32 = 0;

    for n in (2..=n_max).rev() {
        let root = num.nth_root(n);
        if root.pow(n) == num && root < smallest {
            smallest = root;
            exponent = n;
        }
    }

    match smallest == u32::MAX {
        true => None,
        false => Some((smallest, exponent)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_powers_2_to_5() {
        // given
        let lower = 2;
        let upper = 5;

        // when
        let result = distinct_powers(lower, upper);

        // then
        let expected = 15;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_smallest_integer_root_returns_none_for_10() {
        // given
        let num = 10;

        // when
        let result = smallest_integer_root(num);

        // then
        let expected = None;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_smallest_integer_root_returns_3_for_9() {
        // given
        let num = 9;

        // when
        let result = smallest_integer_root(num).unwrap();

        // then
        let expected = (3, 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_smallest_integer_root_returns_2_for_16() {
        // given
        let num = 16;

        // when
        let result = smallest_integer_root(num).unwrap();

        // then
        let expected = (2, 4);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_smallest_integer_root_returns_2_for_64() {
        // given
        let num = 64;

        // when
        let result = smallest_integer_root(num).unwrap();

        // then
        let expected = (2, 6);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_ab_pow_reduce_does_nothing_when_theres_no_integer_root() {
        // given
        let mut num = AbPow {
            base: 3,
            exponent: 5,
        };

        // when
        num.reduce();

        // then
        let expected = AbPow {
            base: 3,
            exponent: 5,
        };
        assert_eq!(expected, num);
    }

    #[test]
    fn test_ab_pow_reduce_makes_base_smaller_and_exponent_larger_when_num_has_integer_root() {
        // given
        let mut num = AbPow {
            base: 4,
            exponent: 2,
        };

        // when
        num.reduce();

        // then
        let expected = AbPow {
            base: 2,
            exponent: 4,
        };
        assert_eq!(expected, num);
    }

    #[test]
    fn test_ab_pow_reduce_makes_base_smaller_and_exponent_larger_when_num_has_integer_root_for_larger_bases(
    ) {
        // given
        let mut num = AbPow {
            base: 64,
            exponent: 2,
        };

        // when
        num.reduce();

        // then
        let expected = AbPow {
            base: 2,
            exponent: 12,
        };
        assert_eq!(expected, num);
    }
}
