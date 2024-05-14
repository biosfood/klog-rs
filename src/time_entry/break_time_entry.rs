use crate::time_entry::{TimeEntry, TimeEntryInfo};

#[derive(Debug)]
pub struct BreakTimeEntry {
    info: TimeEntryInfo,
}

impl TimeEntry for BreakTimeEntry {
    fn get_info(&self) -> &TimeEntryInfo {
        return &self.info
    }

    fn new(text: &str) -> Box<dyn TimeEntry> {
        todo!()
    }

    fn test(text: &str) -> bool {
        todo!()
    }
}
