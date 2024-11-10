use std::fmt;

use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    c_type: TokenType, // type is reserved so c_type, c for custom
    lexeme: String,
    // TODO: will change following
    literal: Option<char>,
    line: u32,
}

impl Token {
    pub fn new(c_type: TokenType, lexeme: String, literal: Option<char>, line: u32) -> Self {
        Token { 
            c_type,
            lexeme,
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal {
            Some(literal) => write!(f, "{:?} {} {}", self.c_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.c_type, self.lexeme),
        }
    }
}