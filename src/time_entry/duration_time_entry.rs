use crate::time_entry::{TimeEntry, TimeEntryInfo};
use chrono::Duration;
use regex::Regex;
use std::cmp::min;

#[derive(Debug)]
pub struct DurationTimeEntry {
    info: TimeEntryInfo,
}

impl TimeEntry for DurationTimeEntry {
    fn get_info(&self) -> &TimeEntryInfo {
        return &self.info;
    }

    fn new(text: &str) -> Box<dyn TimeEntry>
    where
        Self: Sized,
    {
        let regex = Regex::new(r"^\s*(((?<hours>\d+)h\s*((?<minutes>\d{1,2})m)?)|((?<alt_minutes>\d+)m)|((?<alt_hours>\d+)h))").unwrap();
        let result = regex.captures(text).unwrap();

        let minutes = Duration::minutes(
            result
                .name("minutes")
                .or(result.name("alt_minutes"))
                .map_or("0", |x| x.as_str())
                .parse::<i64>()
                .unwrap()
        );
        let hours = Duration::minutes(
            result
                .name("hours")
                .or(result.name("alt_hours"))
                .map_or("0", |x| x.as_str())
                .parse::<i64>()
                .unwrap()
        );
        let duration = minutes + hours;
        return Box::new(DurationTimeEntry {
            info: TimeEntryInfo {
                description: "empty desc".parse().unwrap(),
                duration,
            },
        });
    }

    fn test(text: &str) -> bool
    where
        Self: Sized,
    {
        let regex = Regex::new(r"^\s*((\d+h\s*(\d{1,2}m)?)|(\d+([hm])))").unwrap();
        return regex.is_match(text);
    }
}
