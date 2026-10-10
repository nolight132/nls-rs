use clap::Parser;

use crate::cli::handle;

mod cli;
mod config;
mod format;
mod list;
mod output;
mod view;

fn main() {
    let args = cli::Args::parse();
    handle(&args);
}
