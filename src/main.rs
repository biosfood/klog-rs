mod record;
mod time_entry;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
