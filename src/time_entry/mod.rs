mod break_time_entry;
mod duration_time_entry;

use crate::time_entry::duration_time_entry::DurationTimeEntry;
use chrono::Duration;
use std::fmt::Debug;
use log::info;
use regex::Regex;

#[derive(Debug)]
pub struct TimeEntryInfo {
    description: String,
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
            duration
        }
    }
}

pub trait TimeEntry: Debug {
    fn get_info(&self) -> &TimeEntryInfo;
    fn new(text: &str) -> Box<dyn TimeEntry> where Self: Sized;
    fn test(text: &str) -> bool
    where
        Self: Sized;
}

macro_rules! entry_type {
    ($l: ident) => {
        ($l::test, $l::new)
    };
}

pub fn parse_time_entry(line: &str) -> Option<Box<dyn TimeEntry>> {
    for (test, new) in [entry_type!(DurationTimeEntry)] {
        if (test(line)) {
            return Some(new(line));
        }
    }
    return None;
}
