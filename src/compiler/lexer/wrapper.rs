use std::fmt::{write, Debug};

use super::token::Token;

#[derive(Clone)]
pub struct TokenWrapper {
    pub token: Token,
    line: usize,
    column: usize,
}

impl Debug for TokenWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "🔖 {:?} 📍line:{} col:{}",
        //     self.token,
        //     self.line,
        //     self.column.checked_div(self.line).unwrap_or(self.column)
        // )

        write!(f, "{:?}", self.token)
    }
}
#[allow(dead_code)]
impl TokenWrapper {
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        Self {
            token,
            line,
            column,
        }
    }
    pub fn raw(&self) -> Token {
        return self.token.clone();
    }

    pub fn is(&self, other: Token) -> bool {
        self.token == other
    }
}
