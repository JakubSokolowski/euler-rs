use itertools::Itertools;

pub fn run() {
    let limit = 200_000;
    let digits = [1, 10, 100, 1000, 10000, 100000, 1000000];
    let product = ch_product(&digits, limit);

    println!("Final product: {}", product);
}

fn champernowne_constant(limit: usize) -> String {
    (1..=limit).map(|n| n.to_string()).join("")
}

fn ch_product(digits: &[usize], limit: usize) -> usize {
    let ch_const = champernowne_constant(limit);
    let num_digits = ch_const.len();
    println!("Generated: {} digits", num_digits);
    let last_digit = digits.last().unwrap();
    debug_assert!(*last_digit <= num_digits, "Const to short: {}", num_digits);

    digits
        .iter()
        .map(|d| {
            ch_const
                .chars()
                .nth(d - 1)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch_product_returns_6_for_123() {
        // given
        let digits = [1, 2, 3];
        let limit = 100;

        // when
        let result = ch_product(&digits, limit);

        // then
        let expected = 6;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_ch_product_returns_6_for_1_10_100() {
        // given
        let digits = [1, 10, 100];
        let limit = 100;

        // when
        let result = ch_product(&digits, limit);

        // then
        let expected = 5;
        assert_eq!(expected, result);
    }
}
