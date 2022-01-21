use crate::fractions::Fraction;

pub fn run() {
    let limit = 1000;
    let count = count_longer_nominators(limit);
    println!(
        "There are {} roots below {} limit with longer nominator",
        count, limit
    );
}

pub fn count_longer_nominators(limit: usize) -> usize {
    (1..=limit).filter(has_longer_nominator).count()
}

pub fn has_longer_nominator(limit: &usize) -> bool {
    let root = square_root(*limit);
    root.nominator.to_string().len() > root.denominator.to_string().len()
}

pub fn square_root(limit: usize) -> Fraction {
    let one = Fraction::from_num(1, 1);
    one.add(continued_root(limit))
}

pub fn continued_root(limit: usize) -> Fraction {
    match limit {
        0 => Fraction::from_num(0, 1),
        other => {
            let two = Fraction::from_num(2, 1);
            let next = continued_root(other - 1);
            two.add(next).reciprocal()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(0), Fraction::from_num(1, 1));
        assert_eq!(square_root(1), Fraction::from_num(3, 2));
        assert_eq!(square_root(2), Fraction::from_num(7, 5));
        assert_eq!(square_root(3), Fraction::from_num(17, 12));
        assert_eq!(square_root(4), Fraction::from_num(41, 29));
        assert_eq!(square_root(8), Fraction::from_num(1393, 985));
    }
}
