use clap::Parser;

use crate::{
    list::{ListOptions, Property, stat_dir},
    view::{Column, Table},
};

mod cli;
mod format;
mod list;
mod output;
mod view;

fn main() {
    let args = cli::Args::parse();
    let options = ListOptions::from(&args);
    let columns: Vec<Column> = view::columns(&options);
    let properties: Vec<Property> = columns.iter().filter_map(Column::property).collect();
    let entries = stat_dir(&args.path, &properties, &options).expect("failed to stat");
    let table = Table::new(entries, columns);

    println!("{}", output::build_table(&table));
}
