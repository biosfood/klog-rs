use chrono::Duration;

pub trait TimeEntry {
    fn get_duration(&self) -> Duration;
}