mod break_time_entry;

use std::fmt::{Debug};
use chrono::Duration;

#[derive(Debug)]
pub struct TimeEntryInfo {
    description: String,
    duration: Duration,
}

pub trait TimeEntry: Debug {
    fn get_info(&self) -> &TimeEntryInfo;
    fn new(text: &str) -> Box<dyn TimeEntry> where Self: Sized;
    fn test(text: &str) -> bool where Self: Sized;
}