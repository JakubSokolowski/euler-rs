use lazy_static::lazy_static;
use regex::{Match, Regex};

pub fn extract_words(s: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#""(.*?)""#).unwrap();
    }

    RE.find_iter(s)
        .map(|word: Match| word.as_str().replace('\"', ""))
        .collect()
}
