use clap::Parser;

use crate::{list::stat_dir, output::build_table};

mod cli;
mod format;
mod list;
mod output;

fn main() {
    let args = cli::Args::parse();
    let options = output::OutputOptions::from(&args);
    let contents = stat_dir(&args.path).expect("failed to stat");

    print!("{}", build_table(contents, &options));
}
