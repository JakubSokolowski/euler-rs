pub fn run() {
    let total: usize = (1..=1000).map(|n| count_letters(to_letters(n))).sum();

    println!("Total count: {}", total);
}

pub fn to_letters(num: usize) -> String {
    match num {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        1000 => "one thousand".to_string(),
        _ => to_letters_compound(num),
    }
}

pub fn count_letters(str_number: String) -> usize {
    str_number.replace('-', "").replace(' ', "").len()
}

pub fn to_letters_compound(num: usize) -> String {
    match num {
        21..=99 => {
            let digit_part = num % 10;
            let decimal_part = num - digit_part;
            format!("{}-{}", to_letters(decimal_part), to_letters(digit_part))
        }
        100..=999 => {
            let decimal_part = num % 100;
            let hundreds_part = (num - decimal_part) / 100;
            if decimal_part == 0 {
                format!("{} {}", to_letters(hundreds_part), "hundred")
            } else {
                format!(
                    "{} {} and {}",
                    to_letters(hundreds_part),
                    "hundred",
                    to_letters(decimal_part)
                )
            }
        }
        _ => {
            panic!("Number: {} too large", num);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::p17_number_letter_counts::{count_letters, to_letters};

    #[test]
    fn to_letters_returns_proper_str_representation_for_digit() {
        // given
        let num = 3;

        // when
        let result = to_letters(num);

        // then
        let expected = "three";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_letters_returns_proper_str_representation_for_2_digit_num() {
        // given
        let num = 22;

        // when
        let result = to_letters(num);

        // then
        let expected = "twenty-two";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_letters_returns_proper_str_representation_for_max_2_digit_num() {
        // given
        let num = 99;

        // when
        let result = to_letters(num);

        // then
        let expected = "ninety-nine";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_letters_returns_proper_str_representation_for_3_digit_num() {
        // given
        let num = 112;

        // when
        let result = to_letters(num);

        // then
        let expected = "one hundred and twelve";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_letters_returns_proper_str_representation_for_100() {
        // given
        let num = 100;

        // when
        let result = to_letters(num);

        // then
        let expected = "one hundred";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_letters_returns_proper_str_representation_for_900() {
        // given
        let num = 900;

        // when
        let result = to_letters(num);

        // then
        let expected = "nine hundred";
        assert_eq!(result, expected);
    }

    #[test]
    fn to_letters_returns_proper_str_representation_for_342() {
        // given
        let num = 342;

        // when
        let result = to_letters(num);

        // then
        let expected = "three hundred and forty-two";
        assert_eq!(result, expected);
    }

    #[test]
    fn count_letters_ignores_hyphens_and_spaces() {
        // given
        let num = "three hundred and forty-two";

        // when
        let result = count_letters(num.to_string());

        // then
        let expected = 23;
        assert_eq!(result, expected);
    }
}
