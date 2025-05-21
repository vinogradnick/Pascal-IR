use crate::{compiler::lexer::token::Token, throw_logic, throw_syntax};

use super::{
    ast::{AstNode, ParserResultNode},
    common::Parser,
};

impl Parser {
    pub fn parse_statement(&mut self) -> ParserResultNode {
        self.log_enter("parse_statement");
        if self.expect(Token::Begin, "parse_statement").is_err() {
            self.log_enter("parse_statement:Error->goTo()");
            return self.statement_expr();
        }

        let mut statements = vec![];

        while !self.cur().is(&Token::End) {
            self.log_enter(&format!("BEGIN->STEP {}", statements.len()));

            if self.cur().is_some(&[Token::Period, Token::Begin]) {
                break;
            }

            let statement = self.statement_expr()?;

            statements.push(statement);
            self.log_exit("");
        }
        self.advance_if(Token::End);
        self.log_exit("parse_statement BEGIN");

        return Ok(AstNode::Begin(statements));
    }

    pub fn get_condition(&mut self) -> ParserResultNode {
        self.log_enter("get_condition");

        if self.cur().is(&Token::Odd) {
            self.advance()?;
            let expr = self.get_expr()?;
            return Ok(AstNode::new_unary(Token::Odd, expr));
        }
        let lhs = self.get_expr()?;
        let relop = self.cur();
        let valid_relops = &[
            Token::Eq,
            Token::Neq,
            Token::Lt,
            Token::Lte,
            Token::Gt,
            Token::Gte,
        ];

        if !relop.is_some(valid_relops) {
            return throw_syntax!("invalde");
        }

        let op = relop.clone();
        self.advance()?;

        let rhs = self.get_expr()?;

        self.log_exit("get_condition");
        Ok(AstNode::new_binary(op, lhs, rhs))
        // let op = match self.cur() {
        //     Token::Eq | Token::Neq | Token::Lt | Token::Gt | Token::Gte => self.cur_move(),
        //     other => {
        //         return throw_logic!("ERRR");
        //     }
        // };

        // let rhs = self.get_expr()?;

        // Ok(AstNode::Binary {
        //     op,
        //     lhs: Box::new(lhs),
        //     rhs: Box::new(rhs),
        // })
    }
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn statement_expr(&mut self) -> ParserResultNode {
        self.log_enter("statement_expr");
        let item = self.cur();

        return match item {
            Token::Procedure => self.parse_block("empty"),
            Token::Period => Ok(AstNode::Empty),
            Token::Begin => self.parse_statement(),
            Token::Call => {
                self.advance()?;
                let ident = self.cur();
                self.advance()?;
                self.advance_if(Token::Semicolon);
                self.log_exit("statement_expr::AstNode::CallDecl");
                return Ok(AstNode::CallDecl {
                    ident: ident.get_ident(),
                });
            }
            Token::If => {
                self.advance()?;

                let cond = self.get_condition()?;
                self.expect(Token::Then, "statement_expr::Token::If")?;

                let statement = self.statement_expr()?;

                self.advance_if(Token::Semicolon);

                self.log_exit("statement_expr::AstNode::If ");
                return Ok(AstNode::If {
                    condtion: Box::new(cond),
                    then_statement: Box::new(statement),
                    else_statement: Box::new(AstNode::Empty),
                });
            }
            Token::Read => {
                self.advance()?;
                let nexted = self.get_factor()?;
                self.advance()?;
                self.log_exit("statement_expr::AstNode::ReadDecl");
                return Ok(AstNode::ReadDecl {
                    node: Box::new(nexted),
                });
            }
            Token::Ident(ident) => {
                self.advance()?;
                if self.expect(Token::Assign, "assginment_statement").is_err() {
                    self.log_enter("statement_expr:: Token::Ident :: self.expect");
                }
                self.log_enter("statement_expr::AstNode::AssignStatement");

                let rhs = self.get_expr()?;

                self.advance_if(Token::Semicolon);
                self.log_exit("statement_expr::AstNode::AssignStatement");
                return Ok(AstNode::AssignStatement {
                    lhs: Box::new(AstNode::VariableExpr(ident)),
                    rhs: Box::new(rhs),
                });
            }
            Token::Print => {
                self.advance()?;
                let arg = self.get_expr()?;
                if self.expect(Token::Semicolon, "Token::Print").is_err() {}

                return Ok(AstNode::PrintDecl { arg: Box::new(arg) });
            }
            Token::While => {
                self.advance()?;

                let cond = self.get_condition()?;

                self.log_result(&cond);
                self.expect(Token::Do, "")?;

                let statement = self.statement_expr()?;

                return Ok(AstNode::While {
                    condtion: Box::new(cond),
                    statement: Box::new(statement),
                });
            }

            cur => {
                throw_logic!(format!("not implemented {:?}", cur))
            }
        };
    }
}
