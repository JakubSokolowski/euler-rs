use std::collections::HashSet;

use itertools::Itertools;

pub fn run() {
    let initial = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let triplets: Vec<(usize, usize, usize)> = initial
        .iter()
        .permutations(initial.len())
        .flat_map(|d| {
            let num = d.iter().map(|d| d.to_string()).join("");
            triple_split_permutations(&num).into_iter()
        })
        .filter(|t| is_pandigital_triplet(t))
        .collect();

    let uniq_products: HashSet<usize> = triplets.iter().map(|t| t.2).collect();
    let product_sum: usize = uniq_products.iter().sum();

    println!("Products: {:?} sum: {}", uniq_products, product_sum);
}

pub fn is_pandigital_triplet(triplet: &(usize, usize, usize)) -> bool {
    triplet.0 * triplet.1 == triplet.2
}

pub fn triple_split_permutations(num: &str) -> Vec<(usize, usize, usize)> {
    let min_size = 1;
    let mut res = vec![];
    let max_size = num.len() - 2;

    for a in min_size..=max_size {
        for b in min_size..=(max_size - a + 1) {
            res.push(triple_split(num, a, a + b))
        }
    }

    res
}

pub fn triple_split(num: &str, a: usize, b: usize) -> (usize, usize, usize) {
    (
        num[..a].parse().unwrap(),
        num[a..b].parse().unwrap(),
        num[b..].parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::problems::p32_pandigital_products::{
        is_pandigital_triplet, triple_split, triple_split_permutations,
    };

    #[test]
    fn test_triple_split_splits_num_str_to_nums_by_indices() {
        // given
        let num = "123456789";
        let a = 3;
        let b = 6;

        // when
        let result = triple_split(num, a, b);

        // then
        let expected = (123, 456, 789);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_random_split_returns_split_permutations() {
        // given
        let num = "1234";

        // when
        let result = triple_split_permutations(num);

        // then
        let expected = vec![(1, 2, 34), (1, 23, 4), (12, 3, 4)];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_random_split_returns_split_permutations_for_len_5() {
        // given
        let num = "12345";

        // when
        let result = triple_split_permutations(num);

        // then
        let expected = vec![
            (1, 2, 345),
            (1, 23, 45),
            (1, 234, 5),
            (12, 3, 45),
            (12, 34, 5),
            (123, 4, 5),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_pandigital_triplet_returns_true() {
        // given
        let triplet = (39, 186, 7254);

        // when
        let result = is_pandigital_triplet(&triplet);

        // then
        let expected = true;
        assert_eq!(expected, result);
    }
}

// split iter
// for i 1..len - split.len
