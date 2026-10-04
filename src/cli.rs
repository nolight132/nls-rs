use clap::Parser;
use std::path::PathBuf;

/// Simple program to list files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// List all files
    #[arg(short, long)]
    pub all: bool,

    /// One line output
    #[arg(short = '1', long = "one")]
    pub one: bool,

    /// Path to list
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Plain output
    #[arg(long)]
    pub plain: bool,

    /// Long output
    #[arg(short, long)]
    pub long: bool,
}
