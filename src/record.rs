use crate::time_entry::TimeEntry;
use chrono::{DateTime, Local};
use log::{info, trace};

#[derive(Debug)]
pub struct Record {
    date: DateTime<Local>,
    summary: String,
    entries: Vec<Box<dyn TimeEntry>>,
}

impl Record {
    pub fn load_from_file(filename: &String) -> Vec<Record> {
        info!("trying to read file {filename}");
        let file_content = std::fs::read_to_string(filename)
            .expect(format!("Could not read file {filename}").as_str());
        let lines: Vec<&str> = file_content.split("\n").collect();
        trace!("finished reading file {filename} to RAM, read {} line(s)", lines.len());
        let result = Vec::new();
        return result;
    }
}
