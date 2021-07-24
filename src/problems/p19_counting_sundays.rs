extern crate chrono;
use self::chrono::{Datelike, Weekday};
use chrono::{Date, Duration, TimeZone, Utc};
use std::mem;

struct DateRange(Date<Utc>, Date<Utc>);

impl Iterator for DateRange {
    type Item = Date<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

pub fn run() {
    let start = Utc.ymd(1901, 1, 1);
    let end = Utc.ymd(2000, 12, 31);

    let count = DateRange(start, end)
        .into_iter()
        .filter(|&d| d.day() == 1 && d.weekday() == Weekday::Sun)
        .count();

    println!("Found: {} sundays", count)
}
