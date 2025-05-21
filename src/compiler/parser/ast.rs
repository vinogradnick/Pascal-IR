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

    UnaryAdd(Box<AstNode>),                // +expr
    UnarySub(Box<AstNode>),                // -expr
    BinaryAdd(Box<AstNode>, Box<AstNode>), // lhs + rhs
    BinarySub(Box<AstNode>, Box<AstNode>), // lhs - rhs
    BinaryMul(Box<AstNode>, Box<AstNode>), // lhs * rhs
    BinaryDiv(Box<AstNode>, Box<AstNode>), // lhs / rhs
    Odd(Box<AstNode>),

    BinaryEq(Box<AstNode>, Box<AstNode>),  // =
    BinaryNeq(Box<AstNode>, Box<AstNode>), // #
    BinaryLt(Box<AstNode>, Box<AstNode>),  // <
    BinaryLe(Box<AstNode>, Box<AstNode>),  // <=
    BinaryGt(Box<AstNode>, Box<AstNode>),  // >
    BinaryGe(Box<AstNode>, Box<AstNode>),  // >=

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
    BinaryLte(Box<AstNode>, Box<AstNode>),
    BinaryGte(Box<AstNode>, Box<AstNode>),
}

impl AstNode {
    pub fn new_unary(op: Token, rhs: AstNode) -> AstNode {
        return match op {
            Token::Plus => AstNode::UnaryAdd(Box::new(rhs)),
            Token::Minus => AstNode::UnarySub(Box::new(rhs)),
            Token::Odd => AstNode::Odd(Box::new(rhs)),
            tok => {
                unreachable!("{}", tok)
            }
        };
    }
    pub fn new_binary(op: Token, lhs: AstNode, rhs: AstNode) -> AstNode {
        return match op {
            Token::NonEqual => AstNode::BinaryNeq(Box::new(lhs), Box::new(rhs)),
            Token::Plus => AstNode::BinaryAdd(Box::new(lhs), Box::new(rhs)),
            Token::Minus => AstNode::BinarySub(Box::new(lhs), Box::new(rhs)),
            Token::Eq => AstNode::BinaryEq(Box::new(lhs), Box::new(rhs)),
            Token::Mul => AstNode::BinaryMul(Box::new(lhs), Box::new(rhs)),
            Token::Div => AstNode::BinaryDiv(Box::new(lhs), Box::new(rhs)),
            Token::Lt => AstNode::BinaryLt(Box::new(lhs), Box::new(rhs)),
            Token::Lte => AstNode::BinaryLte(Box::new(lhs), Box::new(rhs)),
            Token::Gt => AstNode::BinaryGt(Box::new(lhs), Box::new(rhs)),
            Token::Gte => AstNode::BinaryGte(Box::new(lhs), Box::new(rhs)),
            _ => {
                panic!("undefiend operation {:?}", op)
            }
        };
    }
}
