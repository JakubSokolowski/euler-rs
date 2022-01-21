use itertools::Itertools;

pub fn iter_digits(num: usize) -> impl Iterator<Item = usize> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn to_digits(num: usize) -> Vec<usize> {
    iter_digits(num).collect()
}

pub fn num_of_digits(num: usize) -> usize {
    num.log10() as usize + 1
}

pub fn from_digits(digits: &[usize]) -> usize {
    digits
        .iter()
        .map(|d| d.to_string())
        .join("")
        .parse()
        .unwrap()
}
