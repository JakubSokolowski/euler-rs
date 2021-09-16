use itertools::Itertools;

pub fn run() {
    let pairs = smallest_difference_pairs(6000);

    let pentagonal: Vec<&(i32, i32)> = pairs
        .iter()
        .filter(|p| is_pentagonal((p.0 + p.1).abs()) && is_pentagonal((p.0 - p.1).abs()))
        .collect();

    let smallest = pentagonal.iter().map(|p| (p.0 - p.1).abs()).min().unwrap();

    println!("{} {:?}", pairs.len(), pentagonal);
    println!("Smallest: {} ", smallest);
}

pub fn nth_pentagonal(n: i32) -> i32 {
    n * (3 * n - 1) / 2
}

pub fn is_pentagonal(num: i32) -> bool {
    let n = (f64::sqrt(24.0 * num as f64 + 1.0) + 1.0) / 6.0;
    n.fract() == 0.0
}

pub fn smallest_difference_pairs(num_pentagonal: i32) -> Vec<(i32, i32)> {
    (1..=num_pentagonal)
        .map(nth_pentagonal)
        .permutations(2)
        .map(|perm| (perm[0], perm[1]))
        .sorted_by(|p1, p2| {
            let p1_diff = p1.1 - p1.0;
            let p2_diff = p2.1 - p2.0;
            Ord::cmp(&p1_diff.abs(), &p2_diff.abs())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_pentagonal_returns_1_for_1() {
        // given
        let num = 1;

        // when
        let result = nth_pentagonal(num);

        // then
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nth_pentagonal_returns_12_for_3() {
        // given
        let num = 3;

        // when
        let result = nth_pentagonal(num);

        // then
        let expected = 12;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smallest_difference_pairs() {
        // given
        let num = 3;

        // when
        let result = smallest_difference_pairs(num);

        // then
        let expected = vec![(1, 5), (5, 1), (5, 12), (12, 5), (1, 12), (12, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_pentagonal_number_returns_false_when_resulting_n_is_integer() {
        // given
        let num = 12;

        // when
        let result = is_pentagonal(num);

        // then
        let expected = true;
        assert_eq!(result, expected);
    }
}
