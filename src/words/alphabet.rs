pub fn word_value(word: &str) -> usize {
    word.chars().map(alphabet_position).sum()
}

pub fn alphabet_position(c: char) -> usize {
    c.to_ascii_uppercase() as usize - 64
}

pub fn is_palindrome(input: &str) -> bool {
    input
        .char_indices()
        .zip(input.char_indices().rev())
        .take_while(|&((first_count, _), (last_count, _))| first_count < last_count)
        .all(|((_, first_char), (_, last_char))| {
            first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
        })
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
