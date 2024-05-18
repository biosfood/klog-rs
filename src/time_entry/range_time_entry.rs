use chrono::NaiveTime;
use regex::Regex;

use crate::time_entry::{TimeEntry, TimeEntryInfo};

#[derive(Debug)]
pub struct RangeTimeEntry {
    info: TimeEntryInfo,
    start: NaiveTime,
    end: NaiveTime,
}

impl TimeEntry for RangeTimeEntry {
    fn get_info(&mut self) -> &mut TimeEntryInfo {
        return &mut self.info;
    }

    fn new(text: &str) -> Box<dyn TimeEntry>
    where Self: Sized {
        let regex = Regex::new(r"^\s*(?<hour_start>\d+)\s*:\s*(?<minute_start>\d+)\s*-\s*(?<hour_end>\d+)\s*:\s*(?<minute_end>\d+)").unwrap();
        let captures = regex.captures(text).unwrap();
        let start = NaiveTime::from_hms_opt(
            captures["hour_start"].parse::<u32>().unwrap(),
            captures["minute_start"].parse::<u32>().unwrap(),
            0,
        )
        .unwrap();
        let end = NaiveTime::from_hms_opt(
            captures["hour_end"].parse::<u32>().unwrap(),
            captures["minute_end"].parse::<u32>().unwrap(),
            0,
        )
        .unwrap();
        let regex = Regex::new(r"^\s*\d+\s*:\s*\d+\s*-\s*\d+\s*:\s*\d+\s*").unwrap();
        return Box::new(RangeTimeEntry {
            start,
            end,
            info: TimeEntryInfo::new(regex.replace(text, "").as_ref(), end - start),
        });
    }

    fn test(text: &str) -> bool
    where Self: Sized {
        let regex = Regex::new(r"^\s*\d+\s*:\s*\d+\s*-\s*\d+\s*:\s*\d+\s*").unwrap();
        return regex.is_match(text);
    }
}
