use chrono::{NaiveDate, Weekday};
use chrono::Datelike;

#[derive(clap::ValueEnum, Clone, Default, Debug, PartialEq)]
pub enum TimeRange {
    Day,
    Week,
    #[default]
    Month,
    Quarter,
    Year,
}

pub fn check_time_range(range: &TimeRange, a: NaiveDate, b: NaiveDate) -> bool {
    match range {
        TimeRange::Day => a == b,
        TimeRange::Week => a.week(Weekday::Sun).first_day() == b.week(Weekday::Sun).first_day(),
        TimeRange::Month => a.month() == b.month(),
        TimeRange::Quarter => a.month() / 3 == b.month() / 3,
        TimeRange::Year => a.year() == b.year(),
    }
}

pub fn format_time_range(range: &TimeRange, date: NaiveDate) -> String {
    match range {
        TimeRange::Year => format!("{}", date.year()),
        TimeRange::Quarter => format!("{}Q{}", date.year(), date.month() / 3),
        TimeRange::Week => format!(
            "{} - {}",
            date.week(Weekday::Sun).first_day(),
            date.week(Weekday::Sun).last_day()
        ),
        TimeRange::Month => format!("{}-{}", date.year(), date.month()),
        TimeRange::Day => format!("{}", date),
    }
}
