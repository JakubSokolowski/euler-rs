use crate::words::alphabet::is_palindrome;

pub fn run() {
    let bound = 1_000_000;
    let sum = sum_palindromes(bound);
    println!("Sum of base palindromes: {}", sum);
}

pub fn sum_palindromes(bound: usize) -> usize {
    (1..bound)
        .filter(|i| is_palindrome(&i.to_string()))
        .filter(|i| is_palindrome(&format!("{:b}", i)))
        .sum()
}
