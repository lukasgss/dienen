use std::{
    borrow::Cow,
    io::{self, Write},
};

use postgresql::{ColumnInfo, Value};

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

pub fn draw_result(cols: &Vec<ColumnInfo>, rows: &Vec<Vec<Value>>) {
    let biggest_value_lengths_for_each_row = get_biggest_value_length_for_each_column(cols, rows);

    print_table_cols(cols, &biggest_value_lengths_for_each_row);
}

fn print_table_cols(cols: &Vec<ColumnInfo>, lengths: &Vec<usize>) {
    print_table_headers(&cols, &lengths);
}

enum RowBuildingPhase {
    TopHeader = 1,
    ColumnName = 2,
    BottomHeader = 3,
}

impl RowBuildingPhase {
    // Amount of rows table header has, for example:
    // +-------------+
    // | Description |
    // +-------------+
    const HEADER_ROWS: u8 = 3;

    const ALL: [Self; Self::HEADER_ROWS as usize] =
        [Self::TopHeader, Self::ColumnName, Self::BottomHeader];
}

#[inline]
fn print_table_headers(columns: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    for phase in RowBuildingPhase::ALL {
        match phase {
            RowBuildingPhase::TopHeader | RowBuildingPhase::BottomHeader => {
                print_top_or_bottom_header_phase(columns, biggest_value_lens)
            }
            RowBuildingPhase::ColumnName => print_column_names_phase(columns, biggest_value_lens),
        }
    }

    _ = io::stdout().flush();
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

    println!();
}

fn print_column_names_phase(cols: &Vec<ColumnInfo>, biggest_value_lens: &Vec<usize>) {
    for (idx, col) in cols.iter().enumerate() {
        print!("|");

        let width = biggest_value_lens[idx].max(col.name.len());
        let padding = width - col.name.chars().count();
        let left = padding / 2;
        let right = padding - left + 1;

        print!("{}", " ".repeat(left + 1));
        print!("{}", col.name);
        print!("{}", " ".repeat(right));
    }

    println!("|");
}

fn get_biggest_value_length_for_each_column(
    col_info: &Vec<ColumnInfo>,
    rows: &Vec<Vec<Value>>,
) -> Vec<usize> {
    let mut biggest_row_values = Vec::<usize>::with_capacity(col_info.len());

    for (idx, _) in col_info.iter().enumerate() {
        let mut biggest_value_len: usize = 0;

        for value in rows {
            let len = value[idx].as_str().len();

            if len > biggest_value_len {
                biggest_value_len = len;
            }
        }

        biggest_row_values.push(biggest_value_len);
    }

    biggest_row_values
}
