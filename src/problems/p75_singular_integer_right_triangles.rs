extern crate test;
use rayon::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    pub fn all_sides_are_integer(&self) -> bool {
        self.a.fract() == 0.0 && self.b.fract() == 0.0 && self.c.fract() == 0.0
    }

    pub fn longest_side(&self) -> f64 {
        f64::max(f64::max(self.a, self.b), f64::max(self.b, self.c))
    }
}

pub fn only_one_integer_sided(wire_len: usize) -> bool {
    let mut upper_bound = wire_len;
    let mut count: usize = 0;

    for i in 1..=wire_len / 2 {
        if i >= upper_bound {
            break;
        }

        let h = i as f64;
        let triangle = get_triangle(wire_len as f64, h);

        if triangle.all_sides_are_integer() {
            count += 1;

            if (triangle.longest_side() as usize) < upper_bound {
                upper_bound = triangle.longest_side() as usize;
            }

            if count > 2 {
                break;
            }
        }
    }

    if count == 2 {
        return true;
    }

    false
}

pub fn get_triangle(perimeter: f64, a: f64) -> Triangle {
    let b = get_other_side(perimeter, a);
    let c = get_missing_side(a, b);

    Triangle { a, b, c }
}

pub fn get_other_side(perimeter: f64, a: f64) -> f64 {
    ((2.0 * a * perimeter) - perimeter * perimeter) / (2.0 * (a - perimeter))
}

pub fn get_missing_side(a: f64, b: f64) -> f64 {
    f64::sqrt(a.powi(2) + b.powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn get_sides_returns_sides() {
        // given
        let perimeter = 12.0;
        let side = 3.0;

        // when
        let res = get_triangle(perimeter, side as f64);

        // then
        let expected = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn all_sides_integer_returns_true_when_all_sides_are_integer() {
        // given
        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };

        // when
        let res = triangle.all_sides_are_integer();

        // then
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn all_sides_integer_returns_true_when_some_sides_are_not_an_integer() {
        // given
        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.5,
        };

        // when
        let res = triangle.all_sides_are_integer();

        // then
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn longest_side_returns_longest_side() {
        // given
        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };

        // when
        let res = triangle.longest_side();

        // then
        let expected = 5.0;
        assert_eq!(res, expected);
    }

    #[test]
    fn find_possible_event_for_120() {
        let wire = 120;
        only_one_integer_sided(wire);
    }

    #[test]
    fn test_only_one_integer_side_returns_true_for_12() {
        // given
        let wire = 12;

        // when
        let result = only_one_integer_sided(wire);

        // then
        let expected = true;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_only_one_integer_side_returns_false_for_13() {
        // given
        let wire = 13;

        // when
        let result = only_one_integer_sided(wire);

        // then
        let expected = false;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_only_one_integer_side_returns_false_for_limit() {
        // given
        let wire = 1500000;

        // when
        let result = only_one_integer_sided(wire);

        // then
        let expected = false;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_only_one_integer_side_returns_false_for_120() {
        // given
        let wire = 120;

        // when
        let result = only_one_integer_sided(wire);

        // then
        let expected = false;
        assert_eq!(expected, result);
    }

    #[bench]
    fn bench_find_possible_even(b: &mut Bencher) {
        let wire = 1500000;
        b.iter(|| only_one_integer_sided(wire));
    }
}

pub fn run() {
    let limit = 1500000;

    let count = (12..=limit)
        .into_par_iter()
        .map(|i| {
            if i % 1000 == 0 {
                println!("Iteration: {}", i)
            }
            only_one_integer_sided(i)
        })
        .filter(|&v| v)
        .count();

    println!("Wire len count: {}", count);
}
