use crate::token::{Literal, Token};

pub struct BinaryExpr<'a> {
    pub(super) left: Box<Expr<'a>>,
    pub(super) operator: Token<'a>,
    pub(super) right: Box<Expr<'a>>,
}

pub struct UnaryExpr<'a> {
    pub(super) operator: Token<'a>,
    pub(super) right: Box<Expr<'a>>,
}

impl<'a> BinaryExpr<'a> {
    fn new(left: Expr<'a>, operator: Token<'a>, right: Expr<'a>) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl<'a> UnaryExpr<'a> {
    fn new(operator: Token<'a>, right: Expr<'a>) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

pub enum Expr<'a> {
    Binary(Box<BinaryExpr<'a>>),
    Unary(Box<UnaryExpr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(Literal<'a>),
}

impl<'a> Expr<'a> {
    pub fn new_binary(left: Expr<'a>, operator: Token<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(left, operator, right)))
    }

    pub fn new_unary(operator: Token<'a>, right: Expr<'a>) -> Self {
        Expr::Unary(Box::new(UnaryExpr::new(operator, right)))
    }

    pub fn new_grouping(expr: Expr<'a>) -> Self {
        Expr::Grouping(Box::new(expr))
    }

    pub fn new_literal(literal: Literal<'a>) -> Self {
        Expr::Literal(literal)
    }
}

// !NOTE: Not sure if I need visitor pattern in Rust.
// pub trait Visitor<'a, T> {
//     fn visit_binary(&mut self, binary_expr: &BinaryExpr<'a>) -> T;
//     fn visit_literal(&mut self, value: &Literal<'a>) -> T;
//     fn visit_grouping(&mut self, expr: &Expr<'a>) -> T;
//     fn visit_unary(&mut self, expr: &UnaryExpr<'a>) -> T;
// }

// impl<'a> Expr<'a> {
//     pub fn accept<T>(&self, visitor: &mut dyn Visitor<'a, T>) -> T {
//         match self {
//             Expr::Binary(expr) => visitor.visit_binary(expr),
//             Expr::Grouping(expr) => visitor.visit_grouping(expr),
//             Expr::Literal(expr) => visitor.visit_literal(expr),
//             Expr::Unary(expr) => visitor.visit_unary(expr),
//         }
//     }
// }
