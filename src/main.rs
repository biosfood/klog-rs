use clap::Parser;
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
}

fn main() {
    let args = Args::parse();
    let env = Env::default()
        .filter_or("LOG_LEVEL", "warn")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let records: Vec<Record> = Record::load_from_file(&args.filename);
    dbg!(records);
}
