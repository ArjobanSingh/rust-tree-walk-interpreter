use crate::lox_error;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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
        self.current >= self.source.chars().count()
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

    // TODO: can be optimised to store original String into char vector
    fn advance(&mut self) -> Option<char> {
        let ch = self.source.chars().nth(self.current);
        self.current += 1;
        ch
    }

    fn add_token(&mut self, c_type: TokenType) {
        self.add_token_with_literal(c_type, None);
    }

    fn add_token_with_literal(&mut self, c_type: TokenType, literal: Option<char>) {
        // User code could contain non-ascii chars, so can't use string slices, need to convert to chars()
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        let new_token = Token::new(c_type, text, literal, self.line);
        self.tokens.push(new_token);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(ch) = self.source.chars().nth(self.current) {
            if ch != expected {
                return false;
            }
            self.current += 1;
            return true;
        }
        // In case of None value
        false
    }
}
