use chrono::{DateTime, Local};
use crate::time_entry::TimeEntry;

pub struct Record {
    date: DateTime<Local>,
    summary: String,
    entries: Vec<Box<dyn TimeEntry>>,
}