use itertools::Itertools;
use std::collections::HashSet;

const ABUNDANT_MAX: usize = 28123;

pub fn run() {
    println!("{:?}", factors(12));

    let all_abundant: Vec<usize> = (1..ABUNDANT_MAX).filter(|&n| is_abundant(n)).collect();

    println!("{}", all_abundant.len());

    let abundant_sums: HashSet<usize> = all_abundant
        .iter()
        .cartesian_product(all_abundant.iter())
        .map(|(x, y)| x + y)
        .collect();

    let all_that_cannot: usize = (1..ABUNDANT_MAX)
        .filter(|&n| !abundant_sums.contains(&n))
        .sum();

    println!("{}", all_that_cannot);
}

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

pub fn factor_sum(num: usize) -> usize {
    factors(num).iter().sum()
}

pub fn is_abundant(num: usize) -> bool {
    factor_sum(num) > num
}
