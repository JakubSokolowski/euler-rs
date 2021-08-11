use crate::positional::digits;

pub fn run() {
    // I guess there is some upper bound for n^5 digits, let's just assume that
    // its below that
    let max = 10000000;
    let power = 5;
    let res: usize = (2..max)
        .filter(|d| *d == digit_powers_sum(&digits::digits(*d), power))
        .sum();

    println!("{:?}", res)
}

fn digit_powers_sum(digits: &[usize], power: usize) -> usize {
    digits.iter().map(|d| d.pow(power as u32)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_test_digits_splits_number_to_its_digits() {
        // given
        let num = 123;

        // when
        let result = digits::digits(num as usize);

        // then
        let expected = vec![1, 2, 3];
        assert_eq!(expected, result);
    }
}
