use crate::words::alphabet::word_value;
use crate::words::file::extract_words;

pub fn run(input: &str) {
    let words = extract_words(input);
    let count = words.iter().filter(|w| is_triangle_word(w)).count();

    println!("Found: {} triangle words", count);
}

fn is_triangle_number(num: usize) -> bool {
    let n = f64::sqrt(2.0 * num as f64 + 0.25) - 0.5;
    n.fract() == 0.0
}

fn is_triangle_word(word: &str) -> bool {
    is_triangle_number(word_value(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triangle_number_returns_false_when_resulting_n_is_float() {
        // given
        let num = 14;

        // when
        let result = is_triangle_number(num);

        // then
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_triangle_number_returns_false_when_resulting_n_is_integer() {
        // given
        let num = 36;

        // when
        let result = is_triangle_number(num);

        // then
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_triangle_word_returns_true_when_word_value_is_triangle_num() {
        // given
        let num = "SKY";

        // when
        let result = is_triangle_word(num);

        // then
        let expected = true;
        assert_eq!(result, expected);
    }
}
