use postgresql::Database;
use rustyline::error::ReadlineError;

use crate::drawing::draw_result;
use crate::input::{Input, Prompt};

pub fn run(database: &dyn Database) -> Result<(), ReadlineError> {
    let mut prompt = Prompt::new(database.database_name())?;

    loop {
        match prompt.ask()? {
            Input::Line(query) => {
                if !query.trim().is_empty() {
                    execute(database, &query);
                }
            }
            Input::Interrupted | Input::Eof => break,
        }
    }

    Ok(())
}

fn execute(database: &dyn Database, query: &str) {
    match database.execute_query_statement(query) {
        Ok(result) => {
            if let Err(err) = draw_result(&result.cols, &result.rows) {
                println!("{err}");
            }
        }
        Err(_) => println!("{}", database.error_message()),
    }
}
