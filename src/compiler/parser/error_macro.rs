#[macro_export]
macro_rules! throw_new {
    ($msg:expr) => {
        Err(super::error::ParserError::NonMatch {
            message: $msg.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        })
    };
}

#[macro_export]
macro_rules! throw_ub {
    ($context:expr) => {
        Err(super::error::ParserError::Unbound {
            context: $context.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        })
    };
}
#[macro_export]
macro_rules! throw_syntax {
    ($msg:expr) => {
        Err(super::error::ParserError::SyntaxError {
            message: $msg.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        })
    };
}
#[macro_export]
macro_rules! throw_eof {
    () => {
        Err(super::error::ParserError::Eof {
            file: file!(),
            line: line!(),
            column: column!(),
        })
    };
}

#[macro_export]
macro_rules! throw_logic {
    ($msg:expr) => {
        Err(super::error::ParserError::Logic {
            message: $msg.to_string(),
            file: file!(),
            line: line!(),
            column: column!(),
        })
    };
}

#[macro_export]
macro_rules! throw_empty {
    () => {
        Err(ParserError::Unnamed {
            file: file!(),
            line: line!(),
            column: column!(),
        })
    };
}
