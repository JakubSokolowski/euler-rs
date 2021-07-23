use itertools::Itertools;

#[derive(Debug)]
struct FlatMatrix {
    width: usize,
    height: usize,
    values: Vec<usize>,
}

struct Coords {
    x: usize,
    y: usize,
}

impl FlatMatrix {
    pub fn new(values: Vec<usize>) -> FlatMatrix {
        let width = (values.len() as f64).sqrt() as usize;
        let height = width;
        FlatMatrix { values, width, height }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        return row * self.width + column;
    }

    fn max_group(&self, size: usize) -> usize {
        vec![
            self.max_vertical(size),
            self.max_horizontal(size),
            self.max_diag_left(size),
            self.max_diag_right(size)
        ].iter().max().unwrap().clone()
    }

    fn get_value(&self, row: usize, column: usize) -> usize {
        match self.values.get(self.get_index(row, column)) {
            Some(v) => v.clone(),
            None => {
                // this is a hack as i cannot be bothered to check bounds
                // and limit the possible groups. if the row/column position
                // is out of bounds, assume that there's a 1 here - this will
                // not change the overall (incomplete) group product
                1
            }
        }
    }

    fn max_vertical(&self, size: usize) -> usize {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| self.vertical_group(x, y, size))
            .max()
            .unwrap()
    }

    fn max_horizontal(&self, size: usize) -> usize {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| self.horizontal_group(x, y, size))
            .max()
            .unwrap()
    }

    fn max_diag_left(&self, size: usize) -> usize {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| self.diagonal_down_left(x, y, size))
            .max()
            .unwrap()
    }

    fn max_diag_right(&self, size: usize) -> usize {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| self.diagonal_down_right(x, y, size))
            .max()
            .unwrap()
    }

    fn vertical_group(&self, row: usize, column: usize, size: usize) -> usize {
        (0..size)
            .map(|n| self.get_value(row + n, column))
            .fold(1, |prod, val| prod * val)
    }


    fn horizontal_group(&self, row: usize, column: usize, size: usize) -> usize {
        (0..size)
            .map(|n| self.get_value(row, column + n))
            .fold(1, |prod, val| prod * val)
    }

    fn diagonal_down_right(&self, row: usize, column: usize, size: usize) -> usize {
        (0..size)
            .map(|n| self.get_value(row + n, column + n))
            .fold(1, |prod, val| prod * val)
    }

    fn diagonal_down_left(&self, row: usize, column: usize, size: usize) -> usize {
        (0..size)
            .map(|n| {
                if  column < n{
                    1
                } else {
                    self.get_value(row + n, column - n)
                }
            })
            .fold(1, |prod, val| prod * val)
    }
}


pub fn run(input: &Vec<String>) {
    let parsed = parse(input);
    let matrix = FlatMatrix::new(parsed);
    let group_size = 4;
    let max_product = matrix.max_group(group_size);
    println!("Max adjacent group product of size {}: {}", group_size, max_product);
}

fn parse(input: &Vec<String>) -> Vec<usize> {
    input
        .iter()
        .map(|line| {
            line.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_group_returns_proper_product_for_group() {
        // given
        let nums = vec![2; 16];
        let matrix = FlatMatrix::new(nums);

        // when
        let res = matrix.vertical_group(0, 0, 2);

        // then
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_horizontal_group_returns_proper_product_for_group_of_4() {
        // given
        let nums = vec![2; 16];
        let matrix = FlatMatrix::new(nums);

        // when
        let res = matrix.vertical_group(0, 0, 4);

        // then
        let expected = 16;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_horizontal_group_returns_proper_product_for_group_of_5() {
        // given
        let nums = vec![2; 16];
        let matrix = FlatMatrix::new(nums);

        // when
        let res = matrix.vertical_group(0, 0, 5);

        // then
        let expected = 16;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_max_vertical() {
        // given
        let nums = (0..16).collect();
        let matrix = FlatMatrix::new(nums);
        println!("{:?}", matrix);

        // when
        let res = matrix.max_vertical(4);

        // then
        let expected = 3465;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_max_horizontal() {
        // given
        let nums = (0..16).collect();
        let matrix = FlatMatrix::new(nums);
        println!("{:?}", matrix);

        // when
        let res = matrix.max_horizontal(4);

        // then
        let expected = 32760;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_max_diag_right() {
        // given
        let nums = (0..16).collect();
        let matrix = FlatMatrix::new(nums);
        println!("{:?}", matrix);

        // when
        let res = matrix.max_diag_right(4);

        // then
        let expected = 750;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_max_diag_left() {
        // given
        let nums = (0..16).collect();
        let matrix = FlatMatrix::new(nums);
        println!("{:?}", matrix);

        // when
        let res = matrix.max_diag_left(4);

        // then
        let expected = 1944;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_max_group() {
        // given
        let nums = (0..16).collect();
        let matrix = FlatMatrix::new(nums);
        println!("{:?}", matrix);

        // when
        let res = matrix.max_group(4);

        // then
        let expected = 32760;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_max_group_size_2() {
        // given
        let nums = (0..16).collect();
        let matrix = FlatMatrix::new(nums);
        println!("{:?}", matrix);

        // when
        let res = matrix.max_group(2);

        // then
        let expected = 210;
        assert_eq!(res, expected);
    }
}

