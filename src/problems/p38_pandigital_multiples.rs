use crate::positional::digits::to_digits;
use itertools::Itertools;

pub fn run() {
    let bound = 1_000_000;

    let max = (1..=bound).filter_map(get_multiple).max().unwrap();

    println!("Max pandigital multiple: {}", max);
}

pub fn get_multiple(num: usize) -> Option<usize> {
    let mut n = 2;
    let mut latest_len = to_digits(num).len();
    let mut max_pandigital: usize = 0;

    while latest_len < 9 {
        let multiple = concat_multiples(num, n);
        latest_len = multiple.len();
        if is_pandigital(&multiple) {
            let new_multiple: usize = multiple.parse().unwrap();

            if new_multiple > max_pandigital {
                max_pandigital = new_multiple;
            }
        }
        n += 1;
    }

    match max_pandigital {
        0 => None,
        _ => Some(max_pandigital),
    }
}

pub fn concat_multiples(num: usize, n_max: usize) -> String {
    (1..=n_max).map(|n| num * n).join("")
}

pub fn is_pandigital(num_str: &str) -> bool {
    if num_str.len() != 9 {
        return false;
    }

    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for d in digits {
        if !num_str.contains(d) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::problems::p38_pandigital_multiples::{get_multiple, is_pandigital};

    #[test]
    fn test_is_pandigital_returns_true_for_str_with_all_digits_occurring_once() {
        // given
        let num_str = "192384576";

        // when
        let result = is_pandigital(num_str);

        // then
        let expected = true;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_pandigital_returns_false_for_str_with_duplicated_digits() {
        // given
        let num_str = "192384577";

        // when
        let result = is_pandigital(num_str);

        // then
        let expected = false;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_pandigital_returns_false_for_str_shorter_than_9() {
        // given
        let num_str = "12345678";

        // when
        let result = is_pandigital(num_str);

        // then
        let expected = false;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_multiple_returns_max_multiple_for_num() {
        // given
        let num = 192;

        // when
        let result = get_multiple(num).unwrap();

        // then
        let expected = 192384576;
        assert_eq!(expected, result);
    }
}
