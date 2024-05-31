use chrono::{Duration, Local, TimeDelta};
use clap::Parser;
use env_logger::Env;
use log::{debug, info};

use crate::arguments::{Args, Command};
use crate::record::{group_records, Record};
use crate::time_range::{check_time_range, format_time_range, TimeRange};

mod arguments;
mod record;
mod time_entry;
mod time_range;

fn main() {
    let args = Args::parse();
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let records: Vec<Record> = Record::load_from_file(&args.filename);
    info!("read {} records", &records.iter().count());
    match args.command {
        Command::Report { time_range } => {
            let now = Local::now().date_naive();
            let filtered_records: Vec<&Record> = records
                .iter()
                .filter(|record| check_time_range(&time_range, now, record.date))
                .collect();
            info!("filtered {} records", &filtered_records.iter().count());
            let group_time_range = match time_range {
                TimeRange::Day => TimeRange::Day,
                TimeRange::Week => TimeRange::Day,
                TimeRange::Month => TimeRange::Day,
                TimeRange::Quarter => TimeRange::Week,
                TimeRange::Year => TimeRange::Month,
            };
            let record_groups: Vec<Vec<&Record>> =
                group_records(filtered_records, &group_time_range);
            debug!("created {} record groups", record_groups.iter().count());
            let group_times = record_groups
                .iter()
                .map(|group| {
                    group
                        .iter()
                        .map(|record| {
                            record
                                .entries
                                .iter()
                                .map(|entry| entry.get_info().duration)
                                .sum::<Duration>()
                        })
                        .sum::<Duration>()
                })
                .collect::<Vec<Duration>>();
            let total_time = group_times.iter().sum::<Duration>();

            for (i, group) in record_groups.iter().enumerate() {
                let group_time = group_times[i];
                println!(
                    "{}: {}h, {}min",
                    format_time_range(&group_time_range, group.first().unwrap().date),
                    group_time.num_hours(),
                    group_time.num_minutes() % 60,
                );
            }
            println!(
                "total: {}h, {}min",
                total_time.num_hours(),
                total_time.num_minutes() % 60
            );
        }
    }
}
