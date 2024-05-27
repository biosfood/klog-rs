use chrono::NaiveDate;
use log::trace;
use regex::Regex;

use crate::time_entry::{parse_time_entry, TimeEntry};
use crate::time_entry::parse_date::parse_date;

#[derive(Debug)]
pub struct Record {
    pub(crate) date: NaiveDate,
    summary: String,
    pub(crate) entries: Vec<Box<dyn TimeEntry>>,
}

impl Record {
    fn new(group: &Vec<&str>) -> Record {
        let date = parse_date(group[0]);
        let mut entries: Vec<Box<dyn TimeEntry>> = Vec::new();
        let mut summary = String::new();
        for line in group.iter().skip(1) {
            let result = parse_time_entry(line);
            if result.is_some() {
                entries.push(result.unwrap());
                continue;
            }
            let new_summary = line.trim();
            if new_summary.is_empty() {
                continue;
            }
            if entries.is_empty() {
                if !summary.is_empty() {
                    summary.push('\n');
                }
                summary.push_str(new_summary);
            } else {
                let current_entry = entries.last_mut().unwrap();
                let info = current_entry.get_info_mut();
                if !info.description.is_empty() {
                    info.description.push('\n');
                }
                info.description.push_str(new_summary);
            }
        }
        return Record {
            date,
            summary,
            entries,
        };
    }

    fn group_records(lines: Vec<&str>) -> Vec<Vec<&str>> {
        let mut result: Vec<Vec<&str>> = Vec::new();
        let mut current: Vec<&str> = Vec::new();
        // allowed date formats are d.m.y, y-m-d and y/m/d
        let record_start_regex = Regex::new(r"((\d{1,2}\.\d{1,2}\.\d{2,4})|(\d{2,4}-\d{1,2}-\d{1,4})|(\d{2,4}/\d{1,2}/\d{1,2}))(\s|$)").unwrap();
        for line in lines {
            if record_start_regex.is_match(line) {
                result.push(current);
                current = Vec::new();
            }
            current.push(line);
        }
        if !current.is_empty() {
            result.push(current);
        }
        return result;
    }

    pub fn load_from_file(filename: &String) -> Vec<Record> {
        let record_start_regex = Regex::new(r"((\d{1,2}\.\d{1,2}\.\d{2,4})|(\d{2,4}-\d{1,2}-\d{1,4})|(\d{2,4}/\d{1,2}/\d{1,2}))(\s|$)").unwrap();
        trace!("trying to read file {filename}");
        let file_content = std::fs::read_to_string(filename)
            .expect(format!("Could not read file {filename}").as_str());
        let lines: Vec<&str> = file_content.split("\n").map(|line| line.trim()).collect();
        trace!(
            "finished reading file {filename} to RAM, read {} line(s)",
            lines.len()
        );
        let groups = Self::group_records(lines);
        let result = groups
            .iter()
            .filter_map(|group| {
                if group.is_empty() || !record_start_regex.is_match(group[0]) {
                    return None;
                }
                return Some(Record::new(group));
            })
            .collect();
        return result;
    }
}
