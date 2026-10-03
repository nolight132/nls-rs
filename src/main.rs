use clap::Parser;

use crate::{format::format_table, list::stat_dir};

mod cli;
mod format;
mod list;

fn main() {
    let args = cli::Args::parse();
    let options = format::FormatOptions::from(&args);
    let contents = stat_dir(&args.path).expect("failed to stat");

    print!("{}", format_table(contents, &options));
}
