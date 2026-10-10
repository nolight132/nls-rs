use crate::{
    config::Options,
    list::Property,
    view::{Column, Table},
};
use std::{borrow::Cow, fmt::Write, iter::repeat_n};
use unicode_width::UnicodeWidthStr;

const MISSING: &str = "-";
const PADDING: usize = 1;

const BORDER_TOP_LEFT: &str = "╭";
const BORDER_TOP_RIGHT: &str = "╮";
const BORDER_TOP_MIDDLE: &str = "┬";
const BORDER_BOTTOM_LEFT: &str = "╰";
const BORDER_BOTTOM_RIGHT: &str = "╯";
const BORDER_BOTTOM_MIDDLE: &str = "┴";
const BORDER_HORIZONTAL: &str = "─";
const BORDER_VERTICAL: &str = "│";
const _BORDER_CROSS: &str = "┼";

#[derive(PartialEq, Eq, Clone, Copy)]
enum Alignment {
    Left,
    Right,
    Center,
}

fn alignment(col: &Column) -> Alignment {
    match col {
        Column::Index => Alignment::Center,
        Column::Property(Property::Size) => Alignment::Right,
        _ => Alignment::Left,
    }
}

enum Edge {
    Top,
    Bottom,
}

pub fn build_table(table: &Table, options: &Options) -> String {
    let mut output = String::new();
    let (values, widths) = prepare(table, options);

    let alignments = alignments(table);

    output.push_str(&build_edge(&table, &widths, Edge::Top));
    for i in 0..table.rows().len() {
        for j in 0..table.columns().len() {
            // i and j are swapped here, reversed order
            let value = &values[j * table.rows().len() + i];

            output.push_str(BORDER_VERTICAL);
            output.extend(repeat_n(' ', PADDING));

            write_aligned(&mut output, value, widths[j], alignments[j]);
            output.extend(repeat_n(' ', PADDING));
        }

        output.push_str(BORDER_VERTICAL);
        output.push('\n');
    }
    output.push_str(&build_edge(&table, &widths, Edge::Bottom));

    output
}

fn write_aligned(output: &mut String, value: &str, width: usize, alignment: Alignment) {
    match alignment {
        Alignment::Left => write!(output, "{:<width$}", value, width = width),
        Alignment::Center => write!(output, "{:^width$}", value, width = width),
        Alignment::Right => write!(output, "{:>width$}", value, width = width),
    }
    .unwrap_or_default();
}

fn prepare<'a>(table: &'a Table, options: &Options) -> (Vec<Cow<'a, str>>, Vec<usize>) {
    let mut values: Vec<Cow<str>> = Vec::new();

    let table = table
        .columns()
        .iter()
        .map(|col| {
            table
                .rows()
                .iter()
                .enumerate()
                .map(|(j, entry)| {
                    let value = col.value(entry, j, options).unwrap_or(MISSING.into());
                    let width = value.width();
                    values.push(value);
                    width
                })
                .max()
                .unwrap_or_default()
        })
        .collect();

    (values, table)
}

fn alignments(table: &Table) -> Vec<Alignment> {
    table.columns().iter().map(|col| alignment(col)).collect()
}

fn build_edge(table: &Table, widths: &[usize], edge: Edge) -> String {
    let mut output = String::new();

    match edge {
        Edge::Top => output.push_str(&BORDER_TOP_LEFT),
        Edge::Bottom => output.push_str(&BORDER_BOTTOM_LEFT),
    }
    for i in 0..table.columns().len() {
        output.push_str(&BORDER_HORIZONTAL.repeat(widths[i] + PADDING * 2));
        if i < table.columns().len() - 1 {
            output.push_str(match edge {
                Edge::Top => &BORDER_TOP_MIDDLE,
                Edge::Bottom => &BORDER_BOTTOM_MIDDLE,
            });
        }
    }

    match edge {
        Edge::Top => output.push_str(&BORDER_TOP_RIGHT),
        Edge::Bottom => output.push_str(&BORDER_BOTTOM_RIGHT),
    }
    output.push('\n');

    output
}
