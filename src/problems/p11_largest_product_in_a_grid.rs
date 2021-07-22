#[derive(Debug)]
struct FlatMatrix {
    width: usize,
    height: usize,
    values: Vec<usize>,
}

impl FlatMatrix {
    pub fn new(values: Vec<usize>) -> FlatMatrix {
        let width = (values.len() as f64).sqrt() as usize;
        let height = width;
        FlatMatrix {values, width, height}
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        return row * self.width + column;
    }

    fn get_value(&self, row: usize, column: usize) -> usize {
        self.values[self.get_index(row, column)]
    }
}


pub fn run(input: &Vec<String>) {
    let parsed = parse(input);
    let matrix = FlatMatrix::new(parsed);

    println!("0, 0: {}", matrix.get_value(0, 0));
    println!("19, 19: {}", matrix.get_value(19, 19));

    println!("{:?}", matrix)
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
