use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(name) => write!(f, "{}", name),
            Token::Number(n) => write!(f, "{}", n),

            // Ключевые слова
            Token::Print => write!(f, "print"),
            Token::NonEqual => write!(f, "#"),
            Token::Read => write!(f, "read"),

            Token::Const => write!(f, "const"),
            Token::Var => write!(f, "var"),
            Token::Procedure => write!(f, "procedure"),
            Token::Call => write!(f, "call"),
            Token::Begin => write!(f, "begin"),
            Token::End => write!(f, "end"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::While => write!(f, "while"),
            Token::Do => write!(f, "do"),
            Token::Odd => write!(f, "odd"),

            // Операторы
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),

            Token::Eq => write!(f, "="),
            Token::Neq => write!(f, "#"),
            Token::Lt => write!(f, "<"),
            Token::Lte => write!(f, "<="),
            Token::Gt => write!(f, ">"),
            Token::Gte => write!(f, ">="),

            // Другие символы
            Token::Assign => write!(f, ":="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Period => write!(f, "."),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),

            Token::EOF => write!(f, "EOF"),
        }
    }
}
