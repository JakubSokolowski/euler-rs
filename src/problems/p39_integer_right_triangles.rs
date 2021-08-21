use crate::problems::p75_singular_integer_right_triangles::get_triangle;

pub fn run() {
    let bound = 1000;
    let max = max_num_integer_triangles(bound);

    println!(
        "Max num of integer triangles found for perimeter: {:?}",
        max
    );
}

pub fn max_num_integer_triangles(bound: usize) -> (usize, usize) {
    (1..=bound)
        .map(|p| (p, num_integer_triangles(p)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

pub fn num_integer_triangles(perimeter: usize) -> usize {
    let mut upper_bound = perimeter;
    let mut count: usize = 0;

    for i in 1..=perimeter / 2 {
        if i >= upper_bound {
            break;
        }

        let h = i as f64;
        let triangle = get_triangle(perimeter as f64, h);

        if triangle.all_sides_are_integer() {
            count += 1;
            if (triangle.longest_side() as usize) < upper_bound {
                upper_bound = triangle.longest_side() as usize;
            }
        }
    }

    // Each combination is counted twice
    // you could probably catch the search space even more,
    // but for all perimeters up to a 1000 its small enough

    count / 2
}

#[cfg(test)]
mod tests {
    use crate::problems::p39_integer_right_triangles::num_integer_triangles;

    #[test]
    fn test_num_integer_triangles_returns_3_for_120() {
        // given
        let perimeter = 120;

        // when
        let result = num_integer_triangles(perimeter);

        // then
        let expected = 3;
        assert_eq!(result, expected);
    }
}
