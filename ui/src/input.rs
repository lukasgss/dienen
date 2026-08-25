use rustyline::{DefaultEditor, error::ReadlineError};

pub enum Input {
    Line(String),
    Interrupted,
    Eof,
}

pub struct Prompt {
    editor: DefaultEditor,
    prompt: String,
}

impl Prompt {
    pub fn new(database_name: &str) -> Result<Self, ReadlineError> {
        Ok(Self {
            editor: DefaultEditor::new()?,
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
