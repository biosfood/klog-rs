use std::fmt::Debug;

use chrono::Duration;
use regex::Regex;

use crate::time_entry::duration_time_entry::DurationTimeEntry;
use crate::time_entry::range_time_entry::RangeTimeEntry;

mod duration_time_entry;
pub(crate) mod parse_date;
mod range_time_entry;

#[derive(Debug)]
pub struct TimeEntryInfo {
    pub(crate) description: String,
    duration: Duration,
}

impl TimeEntryInfo {
    fn new(text: &str, duration: Duration) -> TimeEntryInfo {
        let re = Regex::new(r"^\s*").unwrap();
        let mut description = re.replace(text, "");
        let re = Regex::new(r"\s*$").unwrap();
        let binding = description.to_string();
        description = re.replace(binding.as_str(), "");
        return TimeEntryInfo {
            description: description.to_string(),
            duration,
        };
    }
}

pub trait TimeEntry: Debug {
    fn get_info(&mut self) -> &mut TimeEntryInfo;
    fn new(text: &str) -> Box<dyn TimeEntry>
    where Self: Sized;
    fn test(text: &str) -> bool
    where Self: Sized;
}

macro_rules! entry_type {
    ($l: ident) => {
        (
            $l::test as fn(&str) -> bool,
            $l::new as fn(&str) -> Box<dyn TimeEntry>,
        )
    };
}

pub fn parse_time_entry(line: &str) -> Option<Box<dyn TimeEntry>> {
    for (test, new) in [entry_type!(DurationTimeEntry), entry_type!(RangeTimeEntry)] {
        if test(line) {
            return Some(new(line));
        }
    }
    return None;
}
