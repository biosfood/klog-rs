use clap::{Parser, Subcommand};

use crate::time_range::TimeRange;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // which file to read
    #[arg(required = true, help = "which file to read")]
    pub filename: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    Report {
        #[clap(value_enum, default_value_t = TimeRange::Month, help="The time range to look at")]
        time_range: TimeRange,
    },
}
