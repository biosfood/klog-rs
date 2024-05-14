use crate::time_entry::TimeEntry;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use log::{info, trace};
use regex::Regex;

#[derive(Debug)]
pub struct Record {
    date: NaiveDate,
    summary: String,
    entries: Vec<Box<dyn TimeEntry>>,
}

impl Record {
    fn convert_year(content: &str) -> i32 {
        return if (content.len() == 4) {
            content.parse::<i32>().unwrap()
        } else {
            // TODO: config for this?
            content.parse::<i32>().unwrap() + 2000
        };
    }

    fn new(group: &Vec<&str>) -> Record {
        let extract_date_regex =
            Regex::new(r"((?<first>\d+)[.\-/](?<second>\d+)[.\-/](?<third>\d+))").unwrap();
        let date_data = extract_date_regex.captures(group[0]).unwrap();
        let date = if (group[0].contains(".")) {
            // german standard
            NaiveDate::from_ymd_opt(
                Self::convert_year(date_data.name("third").unwrap().as_str()),
                date_data
                    .name("second")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                date_data
                    .name("first")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            )
        } else {
            NaiveDate::from_ymd_opt(
                Self::convert_year(date_data.name("first").unwrap().as_str()),
                date_data
                    .name("second")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                date_data
                    .name("third")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            )
        }.unwrap();
        return Record {
            date,
            summary: "hello".to_string(),
            entries: vec![],
        }
    }

    fn group_records(lines: Vec<&str>) -> Vec<Vec<&str>> {
        let mut result: Vec<Vec<&str>> = Vec::new();
        let mut current: Vec<&str> = Vec::new();
        // allowed date formats are d.m.y, y-m-d and y/m/d
        let record_start_regex = Regex::new(r"\s*((\d{1,2}\.\d{1,2}\.\d{2,4})|(\d{2,4}-\d{1,2}-\d{1,4})|(\d{2,4}/\d{1,2}/\d{1,2}))\s*").unwrap();
        for line in lines {
            if (record_start_regex.is_match(line)) {
                result.push(current);
                current = Vec::new();
            }
            current.push(line);
        }
        if (!current.is_empty()) {
            result.push(current);
        }
        return result;
    }

    pub fn load_from_file(filename: &String) -> Vec<Record> {
        let record_start_regex = Regex::new(r"\s*((\d{1,2}\.\d{1,2}\.\d{2,4})|(\d{2,4}-\d{1,2}-\d{1,4})|(\d{2,4}/\d{1,2}/\d{1,2}))\s*").unwrap();
        info!("trying to read file {filename}");
        let file_content = std::fs::read_to_string(filename)
            .expect(format!("Could not read file {filename}").as_str());
        let lines: Vec<&str> = file_content.split("\n").collect();
        trace!(
            "finished reading file {filename} to RAM, read {} line(s)",
            lines.len()
        );
        let groups = Self::group_records(lines);
        let result = groups
            .iter()
            .filter_map(|group| {
                if (group.is_empty() || !record_start_regex.is_match(group[0])) {
                    return None;
                }
                return Some(Record::new(group));
            })
            .collect();
        return result;
    }
}
