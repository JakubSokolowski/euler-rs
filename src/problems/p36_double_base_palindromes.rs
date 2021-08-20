pub fn run() {
    let bound = 1_000_000;
    let sum = sum_palindromes(bound);
    println!("Sum of base palindromes: {}", sum);
}

pub fn is_palindrome(input: &str) -> bool {
    input
        .char_indices()
        .zip(input.char_indices().rev())
        .take_while(|&((first_count, _), (last_count, _))| first_count < last_count)
        .all(|((_, first_char), (_, last_char))| {
            first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
        })
}

pub fn sum_palindromes(bound: usize) -> usize {
    (1..bound)
        .filter(|i| is_palindrome(&i.to_string()))
        .filter(|i| is_palindrome(&format!("{:b}", i)))
        .sum()
}
