use clap::Parser;

use crate::cli::handle;

mod cli;
mod color;
mod config;
mod format;
mod git;
mod icons;
mod list;
mod output;
mod view;

fn main() {
    let args = cli::Args::parse();
    handle(&args);
}
