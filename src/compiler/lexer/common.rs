use super::{token::Token, wrapper::TokenWrapper};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 0,
        }
    }
    #[inline]
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.input.len() {
            let ch = self.input[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                self.line += 1;
            }
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_ident_or_keyword(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        match ident.to_lowercase().as_str() {
            "const" => Token::Const,
            "var" => Token::Var,
            "procedure" => Token::Procedure,
            "call" => Token::Call,
            "begin" => Token::Begin,
            "end" => Token::End,
            "if" => Token::If,
            "then" => Token::Then,
            "while" => Token::While,
            "do" => Token::Do,
            "odd" => Token::Odd,
            _ => Token::Ident(ident),
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = 0;
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                number = number * 10 + ch.to_digit(10).unwrap();
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(number)
    }

    pub fn next_token(&mut self) -> TokenWrapper {
        self.skip_whitespace();

        let ch = match self.advance() {
            Some(c) => c,
            None => return TokenWrapper::new(Token::EOF, self.line, self.pos),
        };

        let token = match ch {
            '?' => Token::Read,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mul,
            '/' => Token::Div,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '=' => Token::Eq,
            ',' => Token::Comma,
            '.' => Token::Period,
            '#' => Token::NonEqual,
            ';' => Token::Semicolon,
            '!' => Token::Print,
            ':' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Assign
                } else {
                    panic!("Unexpected character after ':'");
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Lte
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::Neq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Gte
                } else {
                    Token::Gt
                }
            }
            c if c.is_digit(10) => {
                self.pos -= 1;
                self.read_number()
            }
            c if c.is_alphabetic() => {
                self.pos -= 1;
                self.read_ident_or_keyword()
            }
            _ => panic!("Unexpected character: {}", ch),
        };
        return TokenWrapper::new(token, self.line, self.pos);
    }
}
