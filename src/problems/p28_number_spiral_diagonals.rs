pub fn run() {
    let size = 1001;
    let sum = spiral_sum(size);
    println!("Spiral corners sum for size {}: {}", size, sum);
}

pub fn spiral_sum(size: usize) -> usize {
    (1..=size).step_by(2).map(spiral_corners_sum).sum()
}

pub fn spiral_corners_sum(size: usize) -> usize {
    match size {
        1 => 1,
        _ => {
            let prev_top_right_corner = (size - 2).pow(2);
            let increase = size - 1;
            4 * prev_top_right_corner + 10 * increase
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_corners_sum_returns_proper_result_for_size_1() {
        // given
        let size = 1;

        // when
        let sum = spiral_corners_sum(size);

        // then
        let expected = 1;
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_spiral_corners_sum_returns_proper_result_for_size_3() {
        // given
        let size = 3;

        // when
        let sum = spiral_corners_sum(size);

        // then
        let expected = 24;
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_spiral_corners_sum_returns_proper_result_for_size_5() {
        // given
        let size = 5;

        // when
        let sum = spiral_corners_sum(size);

        // then
        let expected = 76;
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_spiral_sum_returns_proper_result_for_size_3() {
        // given
        let size = 3;

        // when
        let sum = spiral_sum(size);

        // then
        let expected = 25;
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_spiral_sum_returns_proper_result_for_size_5() {
        // given
        let size = 5;

        // when
        let sum = spiral_sum(size);

        // then
        let expected = 101;
        assert_eq!(expected, sum);
    }
}
