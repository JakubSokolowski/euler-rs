use std::collections::HashMap;

pub fn run() {
    let longest = (2..=1000)
        .map(|i| (i, find_cycle(i).unwrap_or_else(|| "".to_string())))
        .max_by(|a, b| a.1.len().cmp(&b.1.len()))
        .unwrap();

    println!("{:?}", longest);
}

pub fn find_cycle(denominator: u32) -> Option<String> {
    let numerator: u32 = 1;
    let mut digits = "".to_string();

    let mut lookup: HashMap<u32, u32> = HashMap::new();
    let mut rem = numerator % denominator;

    while rem != 0 && !lookup.contains_key(&rem) {
        lookup.insert(rem, digits.len() as u32);
        rem *= 10;
        let result = rem / denominator;
        digits.push_str(&result.to_string());
        rem %= denominator;
    }

    if rem == 0 {
        return None;
    }

    match lookup.contains_key(&rem) {
        true => {
            let start: usize = *lookup.get(&rem).unwrap() as usize;
            let substr = &digits[start..];
            Some(substr.to_string())
        }
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cycle_returns_none_when_theres_no_cycle() {
        // given
        let denominator = 4;

        // when
        let result = find_cycle(denominator);

        // then
        let expected = None;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_cycle_returns_cycle_for_3() {
        // given
        let denominator = 3;

        // when
        let result = find_cycle(denominator).unwrap();

        // then
        let expected = "3";
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_cycle_returns_cycle_for_6() {
        // given
        let denominator = 6;

        // when
        let result = find_cycle(denominator).unwrap();

        // then
        let expected = "6";
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_cycle_returns_cycle_for_7() {
        // given
        let denominator = 7;

        // when
        let result = find_cycle(denominator).unwrap();

        // then
        let expected = "142857";
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_cycle_returns_cycle_for_9() {
        // given
        let denominator = 9;

        // when
        let result = find_cycle(denominator).unwrap();

        // then
        let expected = "1";
        assert_eq!(expected, result);
    }
}
