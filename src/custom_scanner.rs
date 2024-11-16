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
                _ => (),
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
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        let new_token = Token::new(c_type, text, literal, self.line);
        self.tokens.push(new_token);
    }
}
