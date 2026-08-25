use std::borrow::Cow;

use rustyline::{Completer, Helper, Hinter, Validator, highlight::Highlighter};

const HIGHLIGHTABLE_KEYWORDS: &[&str] = &["select", "from", "where", "update", "delete"];

enum Token {
    Keyword,
    String,
    Number,
    Other,
}

#[derive(Completer, Hinter, Validator, Helper)]
pub struct SqlHighlighter;

impl Highlighter for SqlHighlighter {
    fn highlight<'l>(&self, line: &'l str, _: usize) -> std::borrow::Cow<'l, str> {
        let tokens = tokenize(line);

        if tokens.iter().all(|(kind, _)| matches!(kind, Token::Other)) {
            return Cow::Borrowed(line);
        }

        let mut out = String::with_capacity(line.len() * 2);

        for (kind, text) in tokens {
            match kind {
                Token::Keyword => out.push_str(&paint(text, 238, 206, 159)),
                Token::String => out.push_str(&paint(text, 96, 230, 103)),
                Token::Number => out.push_str(&paint(text, 235, 166, 77)),
                Token::Other => out.push_str(text),
            }
        }

        Cow::Owned(out)
    }

    fn highlight_char(&self, line: &str, _: usize, _: rustyline::highlight::CmdKind) -> bool {
        !line.is_empty()
    }
}

fn paint(text: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m{text}\x1b[0m")
}

fn tokenize(line: &str) -> Vec<(Token, &str)> {
    let bytes = line.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let start = i;

        let kind = match bytes[i] {
            b'\'' | b'"' => {
                i += 1;

                while i < bytes.len() {
                    if bytes[i] == b'\'' || bytes[i] == b'"' {
                        if bytes.get(i + 1) == Some(&b'\'') || bytes.get(i + 1) == Some(&b'"') {
                            i += 2;
                            continue;
                        }

                        i += 1;
                        break;
                    }

                    i += 1;
                }

                Token::String
            }
            b'0'..=b'9' => {
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }

                Token::Number
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }

                if HIGHLIGHTABLE_KEYWORDS.contains(&line[start..i].to_ascii_lowercase().as_str()) {
                    Token::Keyword
                } else {
                    Token::Other
                }
            }
            _ => {
                i += 1;

                while i < bytes.len() && !line.is_char_boundary(i) {
                    i += 1;
                }

                Token::Other
            }
        };

        tokens.push((kind, &line[start..i]));
    }

    tokens
}
