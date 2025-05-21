use crate::compiler::lexer::token::Token;

use super::{
    ast::{AstNode, ParserResultContext, ParserResultNode},
    common::Parser,
};

impl Parser {
    pub fn parse_procedure(&mut self) -> ParserResultNode {
        self.expect(Token::Procedure, "get_procedure")?;

        let ident = self.cur();
        if ident.is_ident() {
            self.advance()?;
        }
        self.advance_if(Token::Semicolon);

        let block = self.parse_program()?;
        let procedure = AstNode::ProcedureDecl {
            name: ident.get_ident(),
            body: Box::new(block),
        };

        self.advance_if(Token::Semicolon);
        return Ok(procedure);
    }
    fn get_procedure(&mut self) -> ParserResultContext<Vec<AstNode>> {
        let mut procedures = vec![];

        while self.cur().is(&Token::Procedure) {
            let proc = self.parse_procedure()?;
            procedures.push(proc);
        }
        return Ok(procedures);
    }

    pub fn parse_program(&mut self) -> ParserResultNode {
        let conts = self.get_const()?;
        let vars = self.get_var()?;
        let procedures = self.get_procedure()?;

        let statement = self.parse_statement()?;
        Ok(AstNode::Program {
            consts: conts,
            vars: vars,
            procedures: procedures,
            statement: Box::new(statement),
        })
    }

    pub fn parse_block(&mut self, name: &str) -> ParserResultNode {
        let conts = self.get_const()?;
        let vars = self.get_var()?;
        let procedures = self.get_procedure()?;
        Ok(AstNode::Block {
            name: name.to_string(),
            consts: conts,
            vars: vars,
            procedures: procedures,
        })
    }

    pub fn get_const(&mut self) -> ParserResultContext<Vec<AstNode>> {
        if self.expect(Token::Const, "get_const").is_err() {
            return Ok(vec![]);
        }
        let mut consts = vec![];

        while !self.cur().is(&Token::Semicolon) {
            let ident = self.cur();
            if !ident.is_ident() {
                self.advance()?;
                continue;
            }
            self.advance()?;
            self.advance()?;
            let value = self.cur();
            consts.push(AstNode::ConstDecl {
                ident: ident.get_ident(),
                val: value.get_number(),
            });
        }
        self.advance()?;

        return Ok(consts);
    }

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    pub fn get_var(&mut self) -> ParserResultContext<Vec<AstNode>> {
        if self.expect(Token::Var, "get_var").is_err() {
            return Ok(vec![]);
        }

        let mut var_decl = vec![];
        while !self.cur().is(&Token::Semicolon) {
            let ident = self.cur();

            if ident.is_ident() {
                var_decl.push(AstNode::VarDecl {
                    ident: ident.get_ident(),
                });
            }
            self.advance()?;
        }
        self.advance()?;
        return Ok(var_decl);
    }
}
