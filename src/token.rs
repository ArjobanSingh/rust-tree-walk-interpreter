use std::fmt::{self, Debug};

use crate::token_type::TokenType;

// #[derive(Debug)]
pub struct Token<'a> {
    c_type: TokenType, // type is reserved so c_type, c for custom
    lexeme: String,
    literal: Option<&'a str>,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(c_type: TokenType, lexeme: String, literal: Option<&'a str>, line: u32) -> Self {
        Token {
            c_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal {
            Some(literal) => write!(f, "{:?} {} {}", self.c_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.c_type, self.lexeme),
        }
    }
}

// Though we could use default Debug.
impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal {
            Some(literal) => write!(f, "{:?} {} {}", self.c_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.c_type, self.lexeme),
        }
    }
}
