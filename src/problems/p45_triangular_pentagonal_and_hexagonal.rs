use crate::problems::p44_pentagon_numbers::is_pentagonal;

pub fn run() {
    let limit = 100000;
    let res = next_valid(limit);

    match res {
        Some(num) => println!("Next triangle, pentagonal and hexagonal num: {:?}", num),
        None => println!("Did not found next num for limit: {}", limit),
    }
}

pub fn next_valid(limit: i32) -> Option<(i32, i32)> {
    for n in 144..=limit {
        let triangle = nth_hexagonal(n);
        if is_pentagonal(triangle) {
            return Some((n, triangle));
        }
    }

    None
}

pub fn nth_hexagonal(n: i32) -> i32 {
    n * (2 * n - 1)
}
