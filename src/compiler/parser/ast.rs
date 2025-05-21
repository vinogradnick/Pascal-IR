use super::error::ParserError;
use crate::compiler::lexer::token::Token;

pub type ParserResultNode = Result<AstNode, ParserError>;

pub type ParserResultContext<T> = Result<T, ParserError>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstNode {
    Program {
        consts: Vec<AstNode>,
        vars: Vec<AstNode>,
        procedures: Vec<AstNode>,
        statement: Box<AstNode>,
    },
    Block {
        name: String,
        consts: Vec<AstNode>,
        vars: Vec<AstNode>,
        procedures: Vec<AstNode>,
    },
    ProcedureDecl {
        name: String,
        body: Box<AstNode>,
    },
    Empty,
    ConstDecl {
        ident: String,
        val: u32,
    },
    VarDecl {
        ident: String,
    },
    Expr(Vec<AstNode>),
    CallDecl {
        ident: String,
    },
    PrintDecl {
        arg: Box<AstNode>,
    },
    ReadDecl {
        node: Box<AstNode>,
    },

    Begin(Vec<AstNode>),

    VariableExpr(String),
    NumericExpr(u32),

    AssignStatement {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },

    Binary {
        op: Token,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Unary {
        op: Token,
        rhs: Box<AstNode>,
    },
    While {
        condtion: Box<AstNode>,
        statement: Box<AstNode>,
    },
    Condition {},
    If {
        condtion: Box<AstNode>,
        then_statement: Box<AstNode>,
        else_statement: Box<AstNode>,
    },
}
