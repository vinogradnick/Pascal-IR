use crate::{
    compiler::{lexer::token::Token, parser::ast::AstNode},
    throw_logic, throw_ub,
};

use super::{ast::ParserResultNode, common::Parser};

impl Parser {
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn get_factor(&mut self) -> ParserResultNode {
        self.log_enter("get_factor");
        let current = self.cur();
        if current.is_ident() {
            self.advance()?;
            self.log_exit("AstNode::VariableExpr");
            return Ok(AstNode::VariableExpr(current.get_ident()));
        }
        if current.is_number() {
            self.advance()?;
            self.log_exit("AstNode::NumericExpr");
            return Ok(AstNode::NumericExpr(current.get_number()));
        }

        if current.is(&Token::LParen) {
            self.advance()?;
            let expr: Result<AstNode, crate::compiler::parser::error::ParserError> =
                self.get_expr();
            self.expect(Token::RParen, "get_factor:RParen")?;
            self.log_exit("Token::LParen");
            return expr;
            // return throw_logic!("LParen not implemented");
        }
        throw_ub!(format!("get_factor:not_impl {:?}", current))
    }
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn get_term(&mut self) -> ParserResultNode {
        self.log_enter("get_term");
        let mut node = self.get_factor()?;

        let state = &[
            Token::Mul,
            Token::Div,
            Token::Gt,
            Token::Gte,
            Token::Eq,
            Token::NonEqual,
        ];
        while self.cur().is_some(state) {
            let op = self.cur();
            self.advance()?;

            let rhs = self.get_factor()?;
            node = AstNode::Binary {
                op: op,
                lhs: Box::new(node),
                rhs: Box::new(rhs),
            }
        }
        self.log_exit("get_term");
        Ok(node)
    }

    pub fn get_expr(&mut self) -> ParserResultNode {
        self.log_enter("get_expr");
        let matched = &[Token::Plus, Token::Minus];

        let current = self.cur();

        let mut node = if current.is_some(matched) {
            let op = current.clone();
            self.advance()?;
            let rhs = self.get_term()?;

            AstNode::Unary {
                op: op,
                rhs: Box::new(rhs),
            }
        } else {
            self.get_term()?
        };

        while self.cur().is_some(matched) {
            let op = self.cur();
            self.advance()?;

            let rhs = self.get_term()?;

            node = AstNode::Binary {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs),
            }
        }

        self.log_exit("get_expr");
        Ok(node)
    }
}
