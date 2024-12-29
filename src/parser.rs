use crate::{
    expression::Expr,
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

// Eg code: 3 + 4 > 6 * (2 - 1) == true
/**
 * expression     → equality ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary | primary ;
 * primary        → NUMBER | STRING | "false" | "true" | "nil" | "(" expression ")" ;
 */

// This is a recursive descent parser
impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr<'a> {
        self.equality()
    }

    fn equality(&mut self) -> Expr<'a> {
        let mut expr = self.comparison();

        while matches!(
            self.peek(),
            Some(token) if [
                TokenType::EqualEqual,
                TokenType::BangEqual,
            ].contains(&token.c_type)
        ) {
            if let Some(operator) = self.advance() {
                let operator = operator.clone();
                let right = self.comparison();
                expr = Expr::new_binary(expr, operator, right);
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr<'a> {
        let mut expr = self.term();

        while matches!(
            self.peek(),
            Some(token) if [
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual
            ].contains(&token.c_type))
        {
            if let Some(operator) = self.advance() {
                let operator = operator.clone();
                let right = self.term();
                expr = Expr::new_binary(expr, operator, right);
            }
        }

        expr
    }

    fn term(&mut self) -> Expr<'a> {
        let mut expr = self.factor();

        while matches!(
            self.peek(),
            Some(token) if [
                TokenType::Minus,
                TokenType::Plus,
            ].contains(&token.c_type)
        ) {
            if let Some(operator) = self.advance() {
                let operator = operator.clone();
                let right = self.factor();
                expr = Expr::new_binary(expr, operator, right);
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr<'a> {
        let mut expr = self.unary();

        while matches!(
            self.peek(),
            Some(token) if [
                TokenType::Slash,
                TokenType::Star,
            ].contains(&token.c_type)
        ) {
            if let Some(operator) = self.advance() {
                let operator = operator.clone();
                let right = self.unary();
                expr = Expr::new_binary(expr, operator, right);
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr<'a> {
        if matches!(
            self.peek(),
            Some(token) if [
                TokenType::Bang,
                TokenType::Minus,
            ].contains(&token.c_type)
        ) {
            if let Some(operator) = self.advance() {
                let operator = operator.clone();
                let right = self.unary();
                return Expr::new_unary(operator, right);
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr<'a> {
        match self.peek() {
            Some(token) if token.c_type == TokenType::False => {
                self.advance();
                return Expr::new_literal(Literal::Bool(false));
            }
            Some(token) if token.c_type == TokenType::True => {
                self.advance();
                return Expr::new_literal(Literal::Bool(true));
            }
            Some(token) if token.c_type == TokenType::Nil => {
                self.advance();
                return Expr::new_literal(Literal::Nil);
            }
            Some(token)
                if token.c_type == TokenType::Number || token.c_type == TokenType::String =>
            {
                if let Some(literal) = token.literal {
                    self.advance();
                    return Expr::new_literal(literal);
                } else {
                    panic!("Literal not found");
                }
            }
            Some(token) if token.c_type == TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(&TokenType::RightParen, "Expect ')' after expression");
                return Expr::new_grouping(expr);
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn consume(&mut self, token: &TokenType, error_msg: &str) -> Expr<'a> {
        todo!("Implement consume")
    }

    pub fn is_at_end(&self) -> bool {
        self.peek()
            .map_or(true, |token| token.c_type == TokenType::Eof)
    }

    pub fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current)
    }

    // consume the current token and return it
    pub fn advance(&mut self) -> Option<&Token<'a>> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn previous(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current - 1)
    }

    // TODO: Remove the following as not needed
    // fn match_token(&mut self, types: &[TokenType]) -> bool {
    //     for token_type in types {
    //         if self.check(token_type) {
    //             self.advance();
    //             return true;
    //         }
    //     }

    //     false
    // }

    // fn check(&self, token_type: &TokenType) -> bool {
    //     self.peek()
    //         .map_or(false, |token| token.c_type == *token_type)
    // }
}
