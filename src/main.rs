use std::cmp::PartialEq;
use std::ops::Deref;

use chrono::{Datelike, Duration, Local, NaiveDate, TimeDelta, Weekday};
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::{debug, info, trace};

use crate::record::Record;

mod record;
mod time_entry;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // which file to read
    #[arg(required = true, help = "which file to read")]
    filename: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, PartialEq)]
enum TimeRange {
    Day,
    Week,
    #[default]
    Month,
    Quarter,
    Year,
}

fn check_time_range(range: &TimeRange, a: NaiveDate, b: NaiveDate) -> bool {
    match range {
        TimeRange::Day => a == b,
        TimeRange::Week => a.week(Weekday::Sun).first_day() == b.week(Weekday::Sun).first_day(),
        TimeRange::Month => a.month() == b.month(),
        TimeRange::Quarter => a.month() / 3 == b.month() / 3,
        TimeRange::Year => a.year() == b.year(),
    }
}

fn format_time_range(range: &TimeRange, date: NaiveDate) -> String {
    match range {
        TimeRange::Year => format!("{}", date.year()),
        TimeRange::Quarter => format!("{}Q{}", date.year(), date.month() / 3),
        TimeRange::Week => format!(
            "{} - {}",
            date.week(Weekday::Sun).first_day(),
            date.week(Weekday::Sun).last_day()
        ),
        TimeRange::Month => format!("{}-{}", date.year(), date.month()),
        TimeRange::Day => format!("{}", date),
    }
}

#[derive(Debug, Subcommand, PartialEq)]
enum Command {
    Report {
        #[clap(value_enum, default_value_t = TimeRange::Month)]
        time_range: TimeRange,
    },
}

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
            let mut record_groups: Vec<Vec<&Record>> = Vec::new();
            for record in filtered_records {
                let mut have_pushed = false;
                for mut group in record_groups.iter_mut() {
                    if check_time_range(&group_time_range, group.first().unwrap().date, record.date)
                    {
                        group.push(record);
                        have_pushed = true;
                    }
                }
                if !have_pushed {
                    record_groups.push(vec![record]);
                }
            }
            debug!("created {} record groups", record_groups.iter().count());
            let mut total = TimeDelta::new(0, 0).unwrap();
            record_groups.iter().for_each(|group| {
                let group_time: Duration = group
                    .iter()
                    .map(|record| {
                        record
                            .entries
                            .iter()
                            .map(|entry| entry.get_info().duration)
                            .sum::<Duration>()
                    })
                    .sum::<Duration>();
                println!(
                    "{}: {}h, {}min",
                    format_time_range(&group_time_range, group.first().unwrap().date),
                    group_time.num_hours(),
                    group_time.num_minutes() % 60,
                );
                total += group_time;
            });
            println!(
                "total: {}h, {}min",
                total.num_hours(),
                total.num_minutes() % 60
            );
        }
    }
}
