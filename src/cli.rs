use crate::{
    config::{Config, Options},
    git::GitStatuses,
    list::{Property, stat_dir},
    output,
    view::{Column, Table, columns},
};
use clap::Parser;
use std::path::PathBuf;

/// Simple program to list files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_version_flag = true)]
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

    /// Version
    #[arg(short, long)]
    pub version: bool,
}

pub fn handle(args: &Args) {
    let config = match Config::load() {
        Ok(c) => c,
        Err(_) => Config::default(),
    };
    let options = Options::from(&args, &config);

    if options.version {
        println!("{}", output::version());
        return;
    }

    let columns = columns(&options);
    let properties: Vec<Property> = columns.iter().filter_map(Column::property).collect();
    let git = GitStatuses::collect(&args.path);
    let entries = match stat_dir(&args.path, &properties, &options) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let table = Table::new(entries, columns, git);

    print!("{}", output::build_table(&table, &options));
}
