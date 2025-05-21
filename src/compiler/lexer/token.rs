#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Number(u32),
    Print,
    NonEqual,
    Read,

    Const,
    Var,
    Procedure,
    Call,
    Begin,
    End,
    If,
    Then,
    While,
    Do,
    Odd,

    Plus,
    Minus,
    Mul,
    Div,

    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,

    Assign, // :=
    Comma,
    Semicolon,
    Period,
    LParen,
    RParen,

    EOF,
}

impl Token {
    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }
    pub fn get_number(&self) -> u32 {
        if let Token::Number(numeric) = self {
            return *numeric;
        }
        return 0;
    }

    pub fn get_ident(&self) -> String {
        if let Token::Ident(numeric) = self {
            return numeric.clone();
        }
        
        return "".to_string();
    }

    pub fn is(&self, other: &Token) -> bool {
        match other {
            Token::Ident(_) => self.is_ident(),
            Token::Number(_) => self.is_number(),
            _ => self == other,
        }
    }
    pub fn is_some(&self, other: &[Token]) -> bool {
        for i in other {
            if self.is(i) {
                return true;
            }
        }
        return false;
    }
}
