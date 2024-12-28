use std::fmt::{self, Debug};

use crate::token_type::TokenType;

#[derive(Clone, Copy)]
pub enum Literal<'a> {
    Str(&'a str),
    Num(f64),
    // Bool(bool),
    // Nil,
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Num(n) => write!(f, "{}", n),
            // Literal::Bool(b) => write!(f, "{}", b),
            // Literal::Nil => write!(f, "nil"),
        }
    }
}

// Though we could use default Debug.
impl<'a> Debug for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Num(n) => write!(f, "{}", n),
            // Literal::Bool(b) => write!(f, "{}", b),
            // Literal::Nil => write!(f, "nil"),
        }
    }
}

pub struct Token<'a> {
    c_type: TokenType, // type is reserved so c_type, c for custom
    lexeme: &'a str,
    literal: Option<Literal<'a>>,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(
        c_type: TokenType,
        lexeme: &'a str,
        literal: Option<Literal<'a>>,
        line: u32,
    ) -> Self {
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
            Some(literal) => write!(f, "{:?} {} {:?}", self.c_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.c_type, self.lexeme),
        }
    }
}

// Though we could use default Debug.
impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal {
            Some(literal) => write!(f, "{:?} {} {:?}", self.c_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.c_type, self.lexeme),
        }
    }
}
