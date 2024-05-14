mod break_time_entry;
mod duration_time_entry;

use crate::time_entry::duration_time_entry::DurationTimeEntry;
use chrono::Duration;
use std::fmt::Debug;
use log::info;

#[derive(Debug)]
pub struct TimeEntryInfo {
    description: String,
    duration: Duration,
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
