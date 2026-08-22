use std::{
    borrow::Cow,
    io::{self, Error, Write},
};

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
    print_table_rows(&rows, &biggest_value_lengths_for_each_row);

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
            RowDrawingPhase::Value => todo!(),
            RowDrawingPhase::BottomHeader => todo!(),
        }
    }
}

fn print_top_or_bottom_header_phase(cols: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    print!("+");

    for (idx, col) in cols.iter().enumerate() {
        let col_biggest_value = biggest_value_lens[idx];

        if col_biggest_value > col.name.len() {
            print!("{}", "-".repeat(col_biggest_value + 2));
        } else {
            print!("{}", "-".repeat(col.name.len() + 2));
        }

        print!("+");
    }

    print!("\n");
}

fn print_column_names_phase(cols: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    for (idx, col) in cols.iter().enumerate() {
        print!("|");

        let width = biggest_value_lens[idx];
        let padding = width - col.name.chars().count();
        let left = padding / 2;
        let right = padding - left + 1;

        print!("{}", " ".repeat(left + 1));
        print!("{}", col.name);
        print!("{}", " ".repeat(right));
    }

    print!("|");
    print!("\n");
}

fn get_biggest_value_length_for_each_column(
    col_info: &Vec<ColumnInfo>,
    rows: &Vec<Vec<Value>>,
) -> Vec<usize> {
    let mut biggest_row_values = Vec::<usize>::with_capacity(col_info.len());

    for (idx, col) in col_info.iter().enumerate() {
        let mut biggest_value_len: usize = 0;

        for value in rows {
            let len = value[idx].as_str().len();

            if len > biggest_value_len {
                biggest_value_len = len;
            }
        }

        if col.name.len() > biggest_value_len {
            biggest_value_len = col.name.len();
        }

        biggest_row_values.push(biggest_value_len);
    }

    biggest_row_values
}
