use crate::{
    list::{ListOptions, Property},
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

pub fn build_table(table: &Table, options: &ListOptions) -> String {
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

            match alignments[j] {
                Alignment::Left => write!(output, "{:<width$}", value, width = widths[j]),
                Alignment::Center => write!(output, "{:^width$}", value, width = widths[j]),
                Alignment::Right => write!(output, "{:>width$}", value, width = widths[j]),
            }
            .unwrap_or_default();
            output.extend(repeat_n(' ', PADDING));
        }

        output.push_str(BORDER_VERTICAL);
        output.push('\n');
    }
    output.push_str(&build_edge(&table, &widths, Edge::Bottom));

    output
}

fn prepare<'a>(table: &'a Table, options: &ListOptions) -> (Vec<Cow<'a, str>>, Vec<usize>) {
    let mut values: Vec<Cow<str>> = Vec::new();

    let table = table
        .columns()
        .iter()
        .enumerate()
        .map(|(i, col)| {
            table
                .rows()
                .iter()
                .enumerate()
                .map(|(j, entry)| {
                    values.push(col.value(entry, j, options).unwrap_or(MISSING.into()));
                    values[i * table.rows().len() + j].width()
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
