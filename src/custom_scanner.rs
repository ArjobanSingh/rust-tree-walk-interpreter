use crate::lox_error;
use crate::token::Token;
use crate::token_type::TokenType;
use std;

pub struct Scanner<'a> {
    source: &'a str,
    source_iter: std::str::CharIndices<'a>,
    tokens: Vec<Token>,
    start: usize,   // keep track of idx of start byte of lexeme
    current: usize, // keep track of idx of current iter byte of lexeme
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            source_iter: source.char_indices(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // we are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        let token = Token::new(TokenType::Eof, String::from(""), None, self.line);
        self.tokens.push(token);

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        // compares byte offsets, no need to decode characters
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        if let Some(ch) = self.advance() {
            match ch {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::SemiColon),
                '*' => self.add_token(TokenType::Star),
                '!' => {
                    let c_type = if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(c_type);
                }
                '=' => {
                    let c_type = if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(c_type);
                }
                '<' => {
                    let c_type = if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(c_type);
                }
                '>' => {
                    let c_type = if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(c_type);
                }
                _ => lox_error(self.line, String::from("Unexpected character.")),
            };
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((idx, ch)) = self.source_iter.next() {
            self.current = idx + ch.len_utf8(); // we could also do += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn add_token(&mut self, c_type: TokenType) {
        self.add_token_with_literal(c_type, None);
    }

    fn add_token_with_literal(&mut self, c_type: TokenType, literal: Option<char>) {
        let text = &self.source[self.start..self.current];
        let new_token = Token::new(c_type, text.to_string(), literal, self.line);
        self.tokens.push(new_token);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(&(idx, ch)) = self.source_iter.clone().peekable().peek() {
            if ch == expected {
                self.source_iter.next();
                self.current = idx + ch.len_utf8();
                return true;
            }
        }

        // In case of None value or not match
        false
    }
}
