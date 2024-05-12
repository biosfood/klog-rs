use std::fmt::Debug;
use chrono::Duration;

pub trait TimeEntry: Debug {
    fn get_duration(&self) -> Duration;
}
