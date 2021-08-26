use crate::words::alphabet::word_value;
use crate::words::file;

pub fn run(input: &str) {
    let mut words = file::extract_words(input);
    words.sort();

    let total: usize = words
        .iter()
        .enumerate()
        .map(|(i, w)| word_value(w) * (i + 1))
        .sum();

    println!("Total sum: {}", total)
}
