use rustyline::{Editor, error::ReadlineError, history::DefaultHistory};

use crate::highlight::SqlHighlighter;

pub enum Input {
    Line(String),
    Interrupted,
    Eof,
}

pub struct Prompt {
    editor: Editor<SqlHighlighter, DefaultHistory>,
    prompt: String,
}

impl Prompt {
    pub fn new(database_name: &str) -> Result<Self, ReadlineError> {
        let mut editor = Editor::new()?;
        editor.set_helper(Some(SqlHighlighter));

        Ok(Self {
            editor,
            prompt: format!("{database_name}> "),
        })
    }

    pub fn ask(&mut self) -> Result<Input, ReadlineError> {
        match self.editor.readline(&self.prompt) {
            Ok(line) => {
                if !line.trim().is_empty() {
                    self.editor.add_history_entry(&line)?;
                }

                Ok(Input::Line(line))
            }
            Err(ReadlineError::Interrupted) => Ok(Input::Interrupted),
            Err(ReadlineError::Eof) => Ok(Input::Eof),
            Err(err) => Err(err),
        }
    }
}
