use std::env;

use env_logger::Env;
use log::info;

use crate::record::Record;

mod record;
mod time_entry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let env = Env::default()
        .filter_or("LOG_LEVEL", "warn")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    if args.len() <= 1 {
        info!("need command line args to run!");
        return;
    }
    let records: Vec<Record> = Record::load_from_file(&args[1]);
    dbg!(records);
}
