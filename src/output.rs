use crate::{
    list::{ListOptions, Property},
    view::{Column, Table},
};
use std::fmt::Write;
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
    let widths = widths(table, options);
    let alignments = alignments(table);

    output.push_str(&build_edge(&table.columns(), &widths, Edge::Top));
    for (i, row) in table.rows().iter().enumerate() {
        for (j, column) in table.columns().iter().enumerate() {
            let value = column.value(row, i, options).unwrap_or(MISSING.to_string());

            output.push_str(BORDER_VERTICAL);
            output.push_str(&" ".repeat(PADDING));

            match alignments[j] {
                Alignment::Left => write!(output, "{:<width$}", value, width = widths[j]),
                Alignment::Center => write!(output, "{:^width$}", value, width = widths[j]),
                Alignment::Right => write!(output, "{:>width$}", value, width = widths[j]),
            }
            .unwrap_or_default();
            output.push_str(&" ".repeat(PADDING));
        }

        output.push_str(BORDER_VERTICAL);
        output.push('\n');
    }
    output.push_str(&build_edge(&table.columns(), &widths, Edge::Bottom));

    output
}

fn widths(table: &Table, options: &ListOptions) -> Vec<usize> {
    table
        .columns()
        .iter()
        .map(|col| {
            table
                .rows()
                .iter()
                .enumerate()
                .map(|(j, entry)| {
                    col.value(entry, j, options)
                        .map_or(MISSING.width(), |v| v.width())
                })
                .max()
                .unwrap_or_default()
        })
        .collect()
}

fn alignments(table: &Table) -> Vec<Alignment> {
    table.columns().iter().map(|col| alignment(col)).collect()
}

fn build_edge(columns: &[Column], widths: &[usize], edge: Edge) -> String {
    let mut output = String::new();

    match edge {
        Edge::Top => output.push_str(&BORDER_TOP_LEFT),
        Edge::Bottom => output.push_str(&BORDER_BOTTOM_LEFT),
    }
    for i in 0..columns.len() {
        output.push_str(&BORDER_HORIZONTAL.repeat(widths[i] + PADDING * 2));
        if i < columns.len() - 1 {
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
