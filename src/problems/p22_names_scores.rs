use lazy_static::lazy_static;
use regex::{Match, Regex};

pub fn run(input: &str) {
    let mut words = extract_words(input);
    words.sort();

    let total: usize = words
        .iter()
        .enumerate()
        .map(|(i, w)| word_value(w) * (i + 1))
        .sum();

    println!("Total sum: {}", total)
}

pub fn alphabet_position(c: char) -> usize {
    c.to_ascii_uppercase() as usize - 64
}

pub fn word_value(word: &str) -> usize {
    word.chars().map(alphabet_position).sum()
}

pub fn extract_words(s: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#""(.*?)""#).unwrap();
    }

    RE.find_iter(s)
        .map(|word: Match| word.as_str().replace("\"", ""))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_position_returns_proper_position() {
        // given
        let letter = 'c';

        // when
        let res = alphabet_position(letter);

        // then
        let expected = 3;
        assert_eq!(res, expected);
    }

    #[test]
    fn word_value_returns_proper_value() {
        // given
        let word = "COLIN";

        // when
        let res = word_value(word);

        // then
        let expected = 53;
        assert_eq!(res, expected);
    }
}
