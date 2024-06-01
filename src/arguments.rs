use chrono::Local;
use chrono::NaiveDate;
use clap::{Parser, Subcommand, value_parser};
use clap::ArgAction;

use crate::time_range::TimeRange;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // which file to read
    #[arg(required = true, help = "which file to read")]
    pub filename: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    Report {
        #[arg(
            help = "One date in the time range to determine the actual time range",
            value_parser = value_parser!(NaiveDate),
            default_value_t = Local::now().date_naive(),
            long = "ref"
        )]
        reference: NaiveDate,

        #[arg(value_enum, default_value_t = TimeRange::Month, help="The time range to look at", long = "range")]
        time_range: TimeRange,

        #[arg(action = ArgAction::SetTrue, long, help="do a nice export to a csv file")]
        csv: bool,
    },
}
