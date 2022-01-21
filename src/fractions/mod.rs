use num::bigint::ToBigInt;
use num::{BigInt, Integer};
use std::ops::{Add, Div, Mul};

#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub nominator: BigInt,
    pub denominator: BigInt,
}

impl Fraction {
    pub fn new(nominator: BigInt, denominator: BigInt) -> Fraction {
        Fraction {
            nominator,
            denominator,
        }
    }

    pub fn from_num(nominator: usize, denominator: usize) -> Fraction {
        Fraction {
            nominator: nominator.to_bigint().unwrap(),
            denominator: denominator.to_bigint().unwrap(),
        }
    }

    pub fn multiply(&self, other: Fraction) -> Fraction {
        let nominator = self.nominator.clone().mul(other.nominator);
        let denominator = self.denominator.clone().mul(other.denominator);
        Fraction {
            nominator,
            denominator,
        }
    }

    pub fn add(&self, other: Fraction) -> Fraction {
        if self.denominator == other.denominator {
            Fraction::new(
                self.nominator.clone().add(other.nominator),
                self.denominator.clone(),
            )
        } else {
            let denominator = self.denominator.clone().mul(other.denominator.clone());
            let nominator = (self
                .nominator
                .clone()
                .mul(denominator.clone())
                .div(self.denominator.clone()))
                + (other.nominator * denominator.clone() / other.denominator);
            Fraction::new(nominator, denominator).simplify()
        }
    }

    pub fn reciprocal(&self) -> Fraction {
        Fraction {
            nominator: self.denominator.clone(),
            denominator: self.nominator.clone(),
        }
    }

    pub fn simplify(&self) -> Fraction {
        let gcd = self.denominator.gcd(&self.nominator);
        let denominator = self.denominator.clone() / gcd.clone();
        let nominator = self.nominator.clone() / gcd;
        Fraction {
            nominator,
            denominator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::ToBigInt;

    #[test]
    fn test_simplify_returns_simplified_fraction_when_fraction_can_be_simplified() {
        // given
        let a = Fraction::new(9.to_bigint().unwrap(), 27.to_bigint().unwrap());

        // when
        let result = a.simplify();

        // then
        let expected = Fraction::new(1.to_bigint().unwrap(), 3.to_bigint().unwrap());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_returns_initial_fraction_when_fraction_cannot_be_simplified() {
        // given
        let a = Fraction::new(4.to_bigint().unwrap(), 7.to_bigint().unwrap());

        // when
        let result = a.simplify();

        // then
        let expected = Fraction::new(4.to_bigint().unwrap(), 7.to_bigint().unwrap());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_add() {
        // given
        let a = Fraction::new(1.to_bigint().unwrap(), 2.to_bigint().unwrap());
        let b = Fraction::new(1.to_bigint().unwrap(), 4.to_bigint().unwrap());

        // when
        let result = a.add(b);

        // then
        let expected = Fraction::new(3.to_bigint().unwrap(), 4.to_bigint().unwrap());
        assert_eq!(result, expected);
    }
}
