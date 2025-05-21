use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    NonMatch {
        message: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    Unnamed {
        file: &'static str,
        line: u32,
        column: u32,
    },
    Unbound {
        context: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    SyntaxError {
        message: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
    Eof {
        file: &'static str,
        line: u32,
        column: u32,
    },
    Logic {
        message: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Unbound {
                context,
                file,
                line,
                column,
            } => {
                write!(
                    f,
                    "Unbound error in {} (at {}:{}:{})",
                    context, file, line, column
                )
            }
            ParserError::SyntaxError {
                message,
                file,
                line,
                column,
            } => {
                write!(
                    f,
                    "Syntax error: {} (at {}:{}:{})",
                    message, file, line, column
                )
            }
            ParserError::Eof { file, line, column } => {
                write!(f, "Unexpected EOF (at {}:{}:{})", file, line, column)
            }
            ParserError::Logic {
                message,
                file,
                line,
                column,
            } => {
                write!(
                    f,
                    "Logic error: {} (at {}:{}:{})",
                    message, file, line, column
                )
            }
            ParserError::NonMatch {
                message,
                file,
                line,
                column,
            } => {
                write!(
                    f,
                    "ParserError: {} (at {}:{}:{})",
                    message, file, line, column
                )
            }
            ParserError::Unnamed { file, line, column } => {
                write!(f, "Unnamed parser error (at {}:{}:{})", file, line, column)
            }
        }
    }
}

impl ParserError {
    pub fn msg(&self) -> String {
        match self {
            ParserError::NonMatch {
                message,
                file,
                line,
                column,
            } => message.clone(),
            ParserError::Unnamed { file, line, column } => String::new(),
            ParserError::Unbound {
                context,
                file,
                line,
                column,
            } => String::new(),
            ParserError::SyntaxError {
                message,
                file,
                line,
                column,
            } => message.clone(),
            ParserError::Eof { file, line, column } => String::new(),
            ParserError::Logic {
                message,
                file,
                line,
                column,
            } => message.clone(),
        }
    }
}

impl std::error::Error for ParserError {}
