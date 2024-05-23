use std::cmp::PartialEq;
use std::ops::Deref;

use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};
use env_logger::Env;

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
        .filter_or("LOG_LEVEL", "warn")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let records: Vec<Record> = Record::load_from_file(&args.filename);
    match args.command {
        Command::Report { time_range } => {
            let now = Local::now().date_naive();
            let filtered_records = records
                .iter()
                .filter(|record| true) // TODO
                .collect::<Vec<&Record>>();
            dbg!(filtered_records);
        }
    }
}
