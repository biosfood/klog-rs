use chrono::{Duration, NaiveTime};
use log::{trace, warn};
use regex::Regex;

use crate::time_entry::{TimeEntry, TimeEntryInfo};

#[derive(Debug)]
pub struct RangeTimeEntry {
    info: TimeEntryInfo,
    start: NaiveTime,
    end: NaiveTime,
}

impl TimeEntry for RangeTimeEntry {
    fn get_info_mut(&mut self) -> &mut TimeEntryInfo {
        return &mut self.info;
    }

    fn get_info(&self) -> &TimeEntryInfo {
        return &self.info;
    }

    fn new(text: &str) -> Box<dyn TimeEntry>
    where Self: Sized {
        let regex = Regex::new(r"^(?<hour_start>\d+)\s*:\s*(?<minute_start>\d+)\s*-\s*((?<hour_end>\d+)\s*:\s*(?<minute_end>\d+)|(?<unknown_end>\?))").unwrap();
        let captures = regex.captures(text).unwrap();
        let start = NaiveTime::from_hms_opt(
            captures["hour_start"].parse::<u32>().unwrap(),
            captures["minute_start"].parse::<u32>().unwrap(),
            0,
        )
        .unwrap();

        let end = if captures.name("unknown_end").is_some() {
            start
        } else {
            NaiveTime::from_hms_opt(
                captures["hour_end"].parse::<u32>().unwrap(),
                captures["minute_end"].parse::<u32>().unwrap(),
                0,
            )
            .unwrap()
        };
        trace!("found a range time entry from {} to {}", start, end);
        if end - start < Duration::new(0, 0).unwrap() {
            warn!(
                "range time entry has a negative duration: {} - {}",
                start, end
            );
        }
        let regex = Regex::new(r"^\d+\s*:\s*\d+\s*-\s*(\d+\s*:\s*\d+|\?)(\s|$)").unwrap();
        return Box::new(RangeTimeEntry {
            start,
            end,
            info: TimeEntryInfo::new(regex.replace(text, "").as_ref(), end - start),
        });
    }

    fn test(text: &str) -> bool
    where Self: Sized {
        let regex = Regex::new(r"^\d+\s*:\s*\d+\s*-\s*(\d+\s*:\s*\d+|\?)(\s|$)").unwrap();
        return regex.is_match(text);
    }
}
