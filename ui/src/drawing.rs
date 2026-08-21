use std::borrow::Cow;

use postgresql::{ColumnInfo, Value};

const RESULT_PADDING: u32 = 1;
const TOTAL_PADDING_FOR_RESULT: u32 = RESULT_PADDING * 2;

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
            Value::Bytes(_) => todo!("find out what to do with this"),
        }
    }
}

pub fn draw_result(cols: &Vec<ColumnInfo>, rows: &Vec<Vec<Value>>) {
    let biggest_value_lengths_for_each_row = get_biggest_value_length_for_each_column(cols, rows);

    println!("{:?}", biggest_value_lengths_for_each_row);
}

fn get_biggest_value_length_for_each_column(
    col_info: &Vec<ColumnInfo>,
    rows: &Vec<Vec<Value>>,
) -> Vec<usize> {
    let mut biggest_row_values = Vec::<usize>::with_capacity(col_info.len());

    for (idx, _) in col_info.iter().enumerate() {
        let mut biggest_value_len: usize = 0;

        for row in &rows[idx] {
            let str_value = row.as_str();

            if str_value.len() > biggest_value_len {
                biggest_value_len = str_value.len();
            }
        }

        biggest_row_values.push(biggest_value_len);
    }

    biggest_row_values
}
