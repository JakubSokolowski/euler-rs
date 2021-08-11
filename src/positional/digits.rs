pub fn iter_digits(num: usize) -> impl Iterator<Item = usize> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn digits(num: usize) -> Vec<usize> {
    iter_digits(num).collect()
}
