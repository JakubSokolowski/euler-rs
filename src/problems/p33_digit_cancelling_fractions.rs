use std::collections::HashSet;

use itertools::Itertools;
use num::ToPrimitive;

use crate::fractions::Fraction;
use crate::positional::digits::to_digits;

pub fn run() {
    let mut fractions: Vec<Fraction> = vec![];

    for n in 10..=98 {
        for d in n + 1..99 {
            let f = Fraction::from_num(n, d);
            if is_naive_simplify_correct(&f) {
                fractions.push(f)
            }
        }
    }

    println!("Correctly simplified: {:?}", fractions);
    let product = fractions.into_iter().reduce(|a, b| a.multiply(b)).unwrap();
    let simple_product = product.simplify();

    println!("Product: {:?}", product);
    println!("Simplified product: {:?}", simple_product);
}

pub fn simplify_naive(fraction: &Fraction) -> Option<Fraction> {
    let nom_digits = to_digits(fraction.nominator.to_usize().unwrap());
    let denom_digits = to_digits(fraction.denominator.to_usize().unwrap());

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

                return Some(Fraction::from_num(*nominator, *denominator));
            }
        }
    }

    None
}

fn first_common(a: &[usize], b: &[usize]) -> Option<usize> {
    let unique_a = a.iter().collect::<HashSet<&usize>>();
    let unique_b = b.iter().collect::<HashSet<&usize>>();
    let common = unique_a.intersection(&unique_b).collect::<Vec<_>>();

    common.first().map(|x| ***x)
}

pub fn is_naive_simplify_correct(fraction: &Fraction) -> bool {
    let naive = simplify_naive(fraction);

    match naive {
        Some(f) => fraction.simplify() == f.simplify(),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::fractions::Fraction;
    use crate::problems::p33_digit_cancelling_fractions::{
        is_naive_simplify_correct, simplify_naive,
    };

    #[test]
    fn test_simplify_naive_returns_fraction_without_common_digit() {
        // given
        let a = Fraction::from_num(49, 98);

        // when
        let result = simplify_naive(&a).unwrap();

        // then
        let expected = Fraction::from_num(4, 8);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_naive_returns_none_when_nom_and_denom_have_no_common_digits() {
        // given
        let a = Fraction::from_num(45, 67);

        // when
        let result = simplify_naive(&a);

        // then
        let expected = None;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simplify_naive_returns_none_for_trivial_cases() {
        // given
        let a = Fraction::from_num(30, 50);

        // when
        let result = simplify_naive(&a);

        // then
        let expected = None;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_naive_simplify_correct_returns_true_when_naive_simplification_returns_correct_result(
    ) {
        // given
        let a = Fraction::from_num(49, 98);

        // when
        let result = is_naive_simplify_correct(&a);

        // then
        let expected = true;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_naive_simplify_correct_returns_false_when_naive_simplification_returns_incorrect_result(
    ) {
        // given
        let a = Fraction::from_num(45, 75);

        // when
        let result = is_naive_simplify_correct(&a);

        // then
        let expected = false;
        assert_eq!(expected, result);
    }
}
