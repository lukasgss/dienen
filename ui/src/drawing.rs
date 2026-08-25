use std::{
    borrow::Cow,
    io::{self, Error, Write},
    str::FromStr,
};

use colored::{ColoredString, Colorize};
use postgresql::{ColumnInfo, Value};

use crate::phases::{HeaderDrawingPhase, RowDrawingPhase};

const RESULT_PADDING: usize = 1;
const TOTAL_PADDING_FOR_RESULT: usize = RESULT_PADDING * 2;

trait ValueExt {
    fn as_str(&self) -> Cow<'_, str>;
}

impl ValueExt for Value {
    fn as_str(&self) -> Cow<'_, str> {
        match self {
            Value::Null => Cow::Borrowed("<null>"),
            Value::Text(text) => Cow::Borrowed(text),
            Value::Int(int) => Cow::Owned(int.to_string()),
            Value::Float(float) => Cow::Owned(float.to_string()),
            Value::Bool(bool) => Cow::Owned(bool.to_string()),
            Value::Uuid(uuid) => Cow::Owned(uuid.to_string()),
            Value::Numeric(numeric) => Cow::Owned(numeric.to_string()),
            Value::TimeStampTz(timestamp) => Cow::Owned(timestamp.to_string()),
            Value::Bytes(_) => todo!("find out what to do with this"),
        }
    }
}

pub fn draw_result(cols: &Vec<ColumnInfo>, rows: &Vec<Vec<Value>>) -> Result<(), Error> {
    let biggest_value_lengths_for_each_row = get_biggest_value_length_for_each_column(cols, rows);

    print_table_cols(cols, &biggest_value_lengths_for_each_row);
    print_table_rows(rows, &biggest_value_lengths_for_each_row);

    io::stdout().flush()
}

fn print_table_cols(cols: &Vec<ColumnInfo>, lengths: &Vec<usize>) {
    print_table_headers(&cols, &lengths);
}

#[inline]
fn print_table_headers(cols: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    for phase in HeaderDrawingPhase::ALL {
        match phase {
            HeaderDrawingPhase::TopHeader | HeaderDrawingPhase::BottomHeader => {
                print_top_or_bottom_header_phase(cols, biggest_value_lens)
            }
            HeaderDrawingPhase::Value => print_column_names_phase(cols, biggest_value_lens),
        }
    }
}

#[inline]
fn print_table_rows(rows: &Vec<Vec<Value>>, biggest_value_lens: &Vec<usize>) {
    for phase in RowDrawingPhase::ROW {
        match phase {
            RowDrawingPhase::Value => print_row_values(rows, biggest_value_lens),
            RowDrawingPhase::BottomHeader => {
                for (idx, row) in rows.iter().enumerate() {
                    if idx == rows.len() - 1 {
                        break;
                    }

                    print_bottom_header_phase_rows(row, biggest_value_lens);
                }
            }
        }
    }
}

fn print_bottom_header_phase_rows(row: &Vec<Value>, biggest_value_lens: &Vec<usize>) {
    print!("+");

    for idx in 0..row.len() {
        let biggest_value = biggest_value_lens[idx];

        print!("{}", "-".repeat(biggest_value + TOTAL_PADDING_FOR_RESULT));

        print!("+");
    }

    print!("\n");
}

fn print_top_or_bottom_header_phase(cols: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    print!("+");

    for idx in 0..cols.len() {
        let len_biggest_value = biggest_value_lens[idx];

        print!("{}", "-".repeat(len_biggest_value + 2));

        print!("+");
    }

    print!("\n");
}

fn print_column_names_phase(cols: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    for (col, biggest_value) in cols.iter().zip(biggest_value_lens) {
        print!("|");

        print_centered_text(&col.name, *biggest_value, Color::Default);
    }

    print!("|");
    print!("\n");
}

fn print_row_values(rows: &Vec<Vec<Value>>, biggest_value_lens: &Vec<usize>) {
    for i in 0..rows.len() {
        print!("|");

        for (value, biggest_value_len) in rows[i].iter().zip(biggest_value_lens) {
            match value {
                Value::Null => {
                    print_centered_text("<null>", *biggest_value_len, Color::BrightYellow)
                }
                Value::Text(text) => print_centered_text(&text, *biggest_value_len, Color::Green),
                Value::Int(int) => {
                    print_centered_text(&int.to_string(), *biggest_value_len, Color::Orange)
                }
                Value::Float(float) => {
                    print_centered_text(&float.to_string(), *biggest_value_len, Color::Orange)
                }
                Value::Bool(bool) => {
                    print_centered_text(&bool.to_string(), *biggest_value_len, Color::Purple)
                }
                Value::Uuid(uuid) => {
                    print_centered_text(&uuid.to_string(), *biggest_value_len, Color::Purple)
                }
                Value::Numeric(decimal) => {
                    print_centered_text(&decimal.to_string(), *biggest_value_len, Color::Orange)
                }
                Value::TimeStampTz(timestamp) => {
                    print_centered_text(timestamp, *biggest_value_len, Color::Blue)
                }
                Value::Bytes(_) => todo!("find out what to do with this"),
            }

            print!("|");
        }

        print!("\n");
    }
}

enum ColorError {
    InvalidColor,
}

enum Color {
    Default,
    Green,
    BrightYellow,
    Yellow,
    Orange,
    Blue,
    Purple,
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yellow" => Ok(Color::Yellow),
            "green" => Ok(Color::Green),
            _ => Err(ColorError::InvalidColor),
        }
    }
}

fn print_centered_text(text: &str, total_width: usize, color: Color) {
    let padding = total_width.saturating_sub(text.chars().count());
    let left = padding / 2;
    let right = padding - left;

    let colored_text: ColoredString;
    match color {
        Color::Default => colored_text = text.normal(),
        Color::Green => colored_text = text.truecolor(96, 230, 103),
        Color::BrightYellow => colored_text = text.truecolor(255, 215, 0),
        Color::Yellow => colored_text = text.truecolor(238, 207, 160),
        Color::Orange => colored_text = text.truecolor(240, 192, 168),
        Color::Blue => colored_text = text.truecolor(143, 199, 255),
        Color::Purple => colored_text = text.truecolor(210, 204, 255),
    }

    print!("{}", " ".repeat(left + 1));
    print!("{}", colored_text);
    print!("{}", " ".repeat(right + 1));
}

fn get_biggest_value_length_for_each_column(
    col_info: &Vec<ColumnInfo>,
    rows: &Vec<Vec<Value>>,
) -> Vec<usize> {
    let mut biggest_row_values = Vec::<usize>::with_capacity(col_info.len());

    for (idx, col) in col_info.iter().enumerate() {
        let mut biggest_value_len: usize = 0;

        for value in rows {
            let len = value[idx].as_str().chars().count();

            if len > biggest_value_len {
                biggest_value_len = len;
            }
        }

        if col.name.chars().count() > biggest_value_len {
            biggest_value_len = col.name.chars().count();
        }

        biggest_row_values.push(biggest_value_len);
    }

    biggest_row_values
}
