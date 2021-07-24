use itertools::Itertools;

pub fn run() {
    let digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = ordered_permutations(&digits, 999999);
    println!("{}", result);
}
// 2783915604

pub fn ordered_permutations(digits: &[u8], num_permutations: usize) -> usize {
    let digits: Vec<_> = digits
        .iter()
        .permutations(digits.len())
        .map(|p| to_number(p.into_iter().cloned().collect()))
        .unique()
        .collect();

    match digits.get(num_permutations) {
        Some(x) => *x,
        None => 0,
    }
}

pub fn to_number(digits: Vec<u8>) -> usize {
    digits
        .iter()
        .map(|&d| d.to_string())
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::problems::p24_lexicographic_permutations::{ordered_permutations, to_number};

    #[test]
    fn to_numbers_converts_digits_to_number() {
        // given
        let digits: Vec<u8> = vec![1, 2, 3, 4];

        // when
        let result = to_number(digits);

        // then
        let expected = 1234;
        assert_eq!(result, expected);
    }

    #[test]
    fn ordered_permutations_returns_permutations() {
        // given
        let digits: Vec<u8> = vec![1, 2, 3, 4];

        // when
        let result = ordered_permutations(&digits, 0);

        // then
        let expected = 1234;
        assert_eq!(result, expected);
    }
}
