use std::collections::HashSet;

use itertools::Itertools;
use num_integer::Integer;

use crate::positional::digits::digits;

#[derive(Debug, PartialEq)]
struct Fraction {
    nominator: usize,
    denominator: usize,
}

impl Fraction {
    pub fn new(nominator: usize, denominator: usize) -> Fraction {
        Fraction {
            nominator,
            denominator,
        }
    }

    pub fn multiply(&self, other: &Fraction) -> Fraction {
        let nominator = self.nominator * other.nominator;
        let denominator = self.denominator * other.denominator;
        Fraction {
            nominator,
            denominator,
        }
    }

    pub fn simplify(&self) -> Fraction {
        let gcd = self.denominator.gcd(&self.nominator);
        let denominator = self.denominator / gcd;
        let nominator = self.nominator / gcd;
        Fraction {
            nominator,
            denominator,
        }
    }

    // 1/2 * 3/5 = 3/15 5/10 * 6/10 = 30/100

    pub fn simplify_naive(&self) -> Option<Fraction> {
        let nom_digits = digits(self.nominator);
        let denom_digits = digits(self.denominator);

        for n in nom_digits.iter() {
            for d in denom_digits.iter() {
                if n == d {
                    let common = first_common(&nom_digits, &denom_digits).unwrap();
                    let is_trivial = common == 0;

                    if is_trivial {
                        return None;
                    }

                    let nominator = nom_digits
                        .iter()
                        .find_or_first(|digit| **digit != common)
                        .unwrap();
                    let denominator = denom_digits
                        .iter()
                        .find_or_first(|digit| **digit != common)
                        .unwrap();
                    return Some(Fraction {
                        nominator: *nominator,
                        denominator: *denominator,
                    });
                }
            }
        }

        None
    }

    pub fn is_naive_simplify_correct(&self) -> bool {
        let naive = self.simplify_naive();

        match naive {
            Some(f) => self.simplify() == f.simplify(),
            None => false,
        }
    }
}

fn first_common(a: &[usize], b: &[usize]) -> Option<usize> {
    let unique_a = a.iter().collect::<HashSet<&usize>>();
    let unique_b = b.iter().collect::<HashSet<&usize>>();
    let common = unique_a.intersection(&unique_b).collect::<Vec<_>>();

    common.first().map(|x| ***x)
}

pub fn run() {
    let mut fractions: Vec<Fraction> = vec![];

    for n in 10..=98 {
        for d in n + 1..99 {
            let f = Fraction::new(n, d);
            if f.is_naive_simplify_correct() {
                fractions.push(f)
            }
        }
    }

    println!("Correctly simplified: {:?}", fractions);
    let product = fractions.into_iter().reduce(|a, b| a.multiply(&b)).unwrap();
    let simple_product = product.simplify();

    println!("Product: {:?}", product);
    println!("Simplified product: {:?}", simple_product);
}

#[cfg(test)]
mod tests {
    use crate::problems::p33_digit_cancelling_fractions::Fraction;

    #[test]
    fn test_simplify_returns_simplified_fraction_when_fraction_can_be_simplified() {
        // given
        let a = Fraction::new(9, 27);

        // when
        let result = a.simplify();

        // then
        let expected = Fraction::new(1, 3);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_returns_initial_fraction_when_fraction_cannot_be_simplified() {
        // given
        let a = Fraction::new(4, 7);

        // when
        let result = a.simplify();

        // then
        let expected = Fraction::new(4, 7);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_naive_returns_fraction_without_common_digit() {
        // given
        let a = Fraction::new(49, 98);

        // when
        let result = a.simplify_naive().unwrap();

        // then
        let expected = Fraction::new(4, 8);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_naive_returns_none_when_nom_and_denom_have_no_common_digits() {
        // given
        let a = Fraction::new(45, 67);

        // when
        let result = a.simplify_naive();

        // then
        let expected = None;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_naive_returns_none_for_trivial_cases() {
        // given
        let a = Fraction::new(30, 50);

        // when
        let result = a.simplify_naive();

        // then
        let expected = None;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_naive_simplify_correct_returns_true_when_naive_simplification_returns_correct_result(
    ) {
        // given
        let a = Fraction::new(49, 98);

        // when
        let result = a.is_naive_simplify_correct();

        // then
        let expected = true;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_naive_simplify_correct_returns_false_when_naive_simplification_returns_incorrect_result(
    ) {
        // given
        let a = Fraction::new(45, 75);

        // when
        let result = a.is_naive_simplify_correct();

        // then
        let expected = false;
        assert_eq!(expected, result);
    }
}
