pub fn e_sieve(bound: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3usize;
    while num * num <= bound {
        let mut j = num * num;
        while j <= bound {
            primes[j] = false;
            j += num;
        }
        num += 2;
    }
    primes
        .into_iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, p)| if p { Some(i) } else { None })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e_sieve_returns_all_primes_below_bound() {
        // given
        let n = 30;

        // when
        let result = e_sieve(n);

        // then
        let expected: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(expected, result);
    }
}
