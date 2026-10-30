use crate::cli::handle;
use clap::Parser;

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
