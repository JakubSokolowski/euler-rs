use crate::primes::factors::factors_uniq;

pub fn run() {
    let group_size = 4;
    let bound = 200_000;

    let group = consecutive_distinct_primes(group_size, bound);

    match group {
        None => {
            println!("Failed to find group for size: {}", group_size);
        }
        Some(res) => {
            eprintln!("Found group: {:?}", res);
        }
    }
}

fn consecutive_distinct_primes(group_size: usize, bound: usize) -> Option<Vec<usize>> {
    for i in 1..bound {
        let nums: Vec<usize> = (i..i + group_size).collect();
        if each_factor_distinct(&nums) {
            return Some(nums);
        }
    }

    None
}

fn each_factor_distinct(nums: &[usize]) -> bool {
    for num in nums {
        let factors = factors_uniq(*num);
        if factors.len() != nums.len() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_distinct_primes_returns_proper_group_for_group_size_2() {
        // given
        let group_size = 2;
        let bound = 100;

        // when
        let result = consecutive_distinct_primes(group_size, bound);

        // then
        let expected = Some(vec![14, 15]);
        assert_eq!(expected, result);
    }

    #[test]
    fn consecutive_distinct_primes_returns_proper_group_for_group_size_3() {
        // given
        let group_size = 3;
        let bound = 1000;

        // when
        let result = consecutive_distinct_primes(group_size, bound);

        // then
        let expected = Some(vec![644, 645, 646]);
        assert_eq!(expected, result);
    }
}
