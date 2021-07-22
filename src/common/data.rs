use std::fs::File;
use std::io::{BufReader, BufRead};

const DEFAULT_CAPACITY: usize = 1000;

pub fn read_to_vec(problem_num: u16) -> Vec<String> {
    let mut lines = Vec::with_capacity(DEFAULT_CAPACITY);
    let data_path = format!("./data/{}.txt", problem_num);
    let file = File::open(&data_path).expect(&format!("Cannot open file at {}", data_path));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    return lines
}
