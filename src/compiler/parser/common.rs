use super::{ast::AstNode, error::ParserError};

use crate::{
    compiler::lexer::{token::Token, wrapper::TokenWrapper},
    throw_eof, throw_syntax,
};

pub struct Parser {
    iter: Vec<TokenWrapper>,
    cursor: usize,
    level: usize,
    enabled_logs: bool,
}
impl Parser {
    pub fn log(&self, msg: &str) {
        if self.enabled_logs {
            let indent = "  ".repeat(self.level);
            eprintln!("{}{}", indent, msg);
        }
    }
    pub fn log_exit(&mut self, context: &str) {
        if self.level > 0 {
            self.level -= 1;
        }
        if self.enabled_logs {
            let msg = format!("← Exit: {}", context);
            self.log(&msg);
        }
    }

    pub fn log_enter(&mut self, context: &str) {
        if self.enabled_logs {
            let token = self.cur(); // захватим заранее, чтобы избежать проблем с borrow
            let msg = format!("→ Enter: {} {:?}", context, token);
            self.log(&msg);
        }
        self.level += 1;
    }

    pub fn log_result(&self, node: &AstNode) {
        if self.enabled_logs {
            eprintln!("{:?}", node);
        }
    }

    pub fn new(items: Vec<TokenWrapper>) -> Self {
        Self {
            iter: items,
            cursor: 0,
            level: 0,
            enabled_logs: true,
        }
    }
    ///
    ///
    ///
    ///
    ///
    #[inline(always)]
    pub fn cur(&self) -> Token {
        return self.iter[self.cursor].raw();
    }
    pub fn rollback(&mut self) {
        self.cursor -= 1;
    }
    pub fn token_line(&self) -> usize {
        return self.iter[self.cursor].line;
    }

    ///
    ///
    ///
    ///
    ///

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn debug(&self) {}
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    ///

    ///
    /// inc cursor->next return Token
    pub fn cur_move(&mut self) -> Token {
        let item = self.cur();
        self.cursor += 1;
        return item;
    }

    pub fn debug_list(&self) {}

    pub fn expect(&mut self, expect: Token, ctx: &str) -> Result<bool, ParserError> {
        if self.cur().is(&expect) {
            self.cursor += 1;
            return Ok(true);
        }
        if ctx.len() > 0 {
            return throw_syntax!(format!("Line {}: {ctx}", self.token_line()));
        }

        throw_syntax!(format!(
            "Line {}: {} missing",
            self.token_line() - 1,
            expect,
        ))
    }

    pub fn advance_if(&mut self, expect: Token) -> bool {
        if self.cur().is(&expect) {
            self.cursor += 1;
            return true;
        }
        return false;
    }

    pub fn advance(&mut self) -> Result<(), ParserError> {
        if self.cursor + 1 >= self.iter.len() {
            return throw_eof!();
        }
        self.cursor += 1;
        Ok(())
    }
}
