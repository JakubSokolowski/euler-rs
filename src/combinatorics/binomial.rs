extern crate num;

use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::One;

pub fn binomial(n: u64, k: u64) -> BigInt {
    let mut res = BigInt::one();
    for i in 0..k {
        res = (res * (n - i).to_bigint().unwrap()) / (i + 1).to_bigint().unwrap();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_5_3() {
        assert_eq!(binomial(5, 3), 10.to_bigint().unwrap());
    }

    #[test]
    fn test_binomial_23_10() {
        assert_eq!(binomial(23, 10), 1144066.to_bigint().unwrap());
        assert_eq!(binomial(23, 11), 1352078.to_bigint().unwrap());
    }
}
