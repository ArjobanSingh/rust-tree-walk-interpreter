use std;

use crate::lox_error;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    source_iter: std::str::CharIndices<'a>,
    tokens: Vec<Token<'a>>,
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

        let token = Token::new(TokenType::Eof, "", None, self.line);
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
                '/' => {
                    // We have found a comment, so we keep consuming the whole line and shunt those chars
                    if self.match_char('/') {
                        // A comment goes until the end of the line.
                        while let Some((_, ch)) = self.peek() {
                            if ch == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else if self.match_char('*') {
                        // We have found a block comment, so we keep consuming the whole block comment
                        while let Some((_, ch)) = self.peek() {
                            if ch == '\n' {
                                self.line += 1;
                            }

                            // We have found the end of the comment
                            if ch == '*'
                                && self
                                    .peek_next()
                                    .map_or(false, |(_, next_ch)| next_ch == '/')
                            {
                                // Consume the closing */
                                self.advance();
                                self.advance();
                                break;
                            };
                            self.advance();
                        }

                        // In case while loop exited because source reached end comment is not closed
                        if self.is_at_end() {
                            lox_error(self.line, "Unterminated block comment.");
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                // Consume and ignore these white space chars
                ' ' | '\r' | '\t' => (),
                // Consume and ignore new line char and just move the line by 1
                '\n' => self.line += 1,
                '"' => self.string(),
                // If we find a digit, we consume the whole number
                '0'..='9' => self.number(),
                _ => {
                    if self.is_alpha(ch) {
                        self.identifier();
                    } else {
                        lox_error(self.line, "Unexpected character.");
                    }
                }
            };
        }
    }

    fn add_token(&mut self, c_type: TokenType) {
        self.add_token_with_literal(c_type, None);
    }

    fn add_token_with_literal(&mut self, c_type: TokenType, literal: Option<Literal<'a>>) {
        let text = &self.source[self.start..self.current];
        let new_token = Token::new(c_type, text, literal, self.line);
        self.tokens.push(new_token);
    }

    fn number(&mut self) {
        while let Some((_, ch)) = self.peek() {
            if !self.is_digit(ch) {
                break;
            }
            self.advance();
        }

        // Look for a fractional part.
        if self.peek().map_or(false, |(_, ch)| ch == '.')
            && self
                .peek_next()
                .map_or(false, |(_, next_ch)| self.is_digit(next_ch))
        {
            // Consume the "."
            self.advance();

            while let Some((_, ch)) = self.peek() {
                if !self.is_digit(ch) {
                    break;
                }
                self.advance();
            }
        }

        // We know start pointer is at start of number and current is at idx after the last digit
        // so we will not trim within the byte.
        let value = &self.source[self.start..self.current];
        self.add_token_with_literal(
            TokenType::Number,
            Some(Literal::Num(value.parse().unwrap())),
        );
    }

    fn string(&mut self) {
        while let Some((_, ch)) = self.peek() {
            // Reached end of string, break loop
            if ch == '"' {
                break;
            }

            // We support multi line strings
            if ch == '\n' {
                self.line += 1;
            }

            // keep consuming string literal chars before the terminating "
            self.advance();
        }

        // In case while loop exited because source reached end and
        // not the string end, show error to the user
        if self.is_at_end() {
            lox_error(self.line, "Unterminated string.");
            return;
        }

        // Now consume the last "
        self.advance();

        // Trim the surroding quotes and we know start is at start quote
        // and current is at idx after the last quote, so we will not trim within the byte.
        // NOTE: We don't support escape sequences as of now in strings.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_literal(TokenType::String, Some(Literal::Str(value)));
    }

    fn identifier(&mut self) {
        while let Some((_, ch)) = self.peek() {
            if !self.is_alpha_numeric(ch) {
                break;
            }
            self.advance();
        }

        match &self.source[self.start..self.current] {
            "and" => self.add_token(TokenType::And),
            "class" => self.add_token(TokenType::Class),
            "else" => self.add_token(TokenType::Else),
            "false" => self.add_token(TokenType::False),
            "for" => self.add_token(TokenType::For),
            "fun" => self.add_token(TokenType::Fun),
            "if" => self.add_token(TokenType::If),
            "nil" => self.add_token(TokenType::Nil),
            "or" => self.add_token(TokenType::Or),
            "print" => self.add_token(TokenType::Print),
            "return" => self.add_token(TokenType::Return),
            "super" => self.add_token(TokenType::Super),
            "this" => self.add_token(TokenType::This),
            "true" => self.add_token(TokenType::True),
            "var" => self.add_token(TokenType::Var),
            "while" => self.add_token(TokenType::While),
            _ => self.add_token(TokenType::Identifier),
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

    // we don't need to explicitly check for is_empty() as peek() does that implicitly
    fn match_char(&mut self, expected: char) -> bool {
        if let Some((_, ch)) = self.peek() {
            if ch == expected {
                self.advance();
                return true;
            }
        }

        // In case of None value or not match
        false
    }

    // small wrapper to get the values without writing again.
    // though can be used directly where needed.
    fn peek(&self) -> Option<(usize, char)> {
        self.source_iter.clone().peekable().peek().copied()
    }

    fn peek_next(&self) -> Option<(usize, char)> {
        let mut iter = self.source_iter.clone().peekable();
        iter.next();
        iter.peek().copied()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}
