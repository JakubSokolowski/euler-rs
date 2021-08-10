pub fn factors(num: usize) -> Vec<usize> {
    let mut i: usize = 1;
    let mut res: Vec<usize> = vec![];

    while i <= (num as f64).sqrt() as usize {
        if num % i == 0 {
            res.push(i);
            if num / i != i {
                res.push(num / i);
            }
        }

        i += 1;
    }
    res.into_iter().filter(|&d| d != 1 && d != num).collect()
}
