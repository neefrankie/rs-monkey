use std::fmt;
use crate::token;
use super::{Node, Expression};


impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Ident(identifier) => identifier.to_string(),
            Expression::IntegerLiteral { 
                token, 
                .. 
            } => token.literal.clone(),
            Expression::Prefix{
                token, 
                ..
            } => token.literal.clone(),
            Expression::Infix{
                token, 
                ..
            } => token.literal.clone(),
            Expression::Boolean {
                token,
                ..
            } => token.literal.clone(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ident(
                identifier
            ) => write!(f, "{}", identifier),
            Expression::IntegerLiteral {
                token, 
                ..
            } => write!(f, "{}", token.literal),
            Expression::Prefix{
                operator, 
                right,
                ..
            } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::Boolean {
                token,
                ..
            } => write!(f, "{}", token.literal),
        }
    }
}

impl Expression {
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Expression::Ident(identifier) => Some(identifier),
            _ => None,
        }
    }

    pub fn as_integral(&self) -> Option<i64> {
        match self {
            Expression::IntegerLiteral {
                value,
                ..
            } => Some(*value),
            _ => None,
        }
    }

    pub fn as_prefix(&self) -> Option<(String, &Expression)> {
        match self {
            Expression::Prefix {
                operator,
                right,
                ..
            } => Some((
                operator.clone(),
                right.as_ref(),
            )),
            _ => None,
        }
    }

    pub fn as_infix(&self) -> Option<(&Expression, String, &Expression)> {
        match self {
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => Some((
                left.as_ref(),
                operator.clone(),
                right.as_ref(),
            )),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String, // token.literal
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

