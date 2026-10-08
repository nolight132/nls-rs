use clap::Parser;

use crate::{
    list::{ListOptions, Property, stat_dir},
    view::{Column, Table},
};

mod cli;
mod config;
mod format;
mod list;
mod output;
mod view;

fn main() {
    let args = cli::Args::parse();
    let config = config::Config::default();
    let options = ListOptions::from(&args, &config);
    if options.version {
        println!(
            r#" _   _ _     ____
| \ | | |   / ___|
|  \| | |   \___ \
|   | | |___ ___) |
|   |_|_____|____/  by nolight132

version 0.1.0"#
        );
        return;
    }
    let columns: Vec<Column> = view::columns(&options);
    let properties: Vec<Property> = columns.iter().filter_map(Column::property).collect();
    let entries = match stat_dir(&args.path, &properties, &options) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let table = Table::new(entries, columns);

    println!("{}", output::build_table(&table, &options));
}
