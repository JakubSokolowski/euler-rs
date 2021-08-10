pub fn is_prime(n: i32) -> bool {
    let limit = (n as f64).sqrt() as i32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}
