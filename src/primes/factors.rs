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

pub fn factors_uniq(num: usize) -> Vec<usize> {
    if num <= 1 {
        return vec![];
    }

    let mut factors: Vec<usize> = Vec::new();
    let mut curr = num;

    loop {
        let factor = first_factor(curr);
        factors.push(factor);

        if curr == factor {
            break;
        }

        while curr % factor == 0 {
            curr /= factor;
        }

        if curr == 1 {
            break;
        }
    }

    factors
}

fn first_factor(num: usize) -> usize {
    if num % 2 == 0 {
        return 2;
    }

    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= num) {
        if num % n == 0 {
            return n;
        }
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_uniq_returns_unique_factors_of_given_num() {
        // given
        let num = 645;

        // when
        let factors = factors_uniq(num);

        // then
        let expected: Vec<usize> = vec![3, 5, 43];
        assert_eq!(expected, factors);
    }
}
