use std::fmt;
use crate::token;
use super::{Node, Expression};


impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Ident(identifier) => identifier.to_string(),
            Expression::IntegerLiteral { 
                token, 
                value: _ 
            } => token.literal.clone(),
            Expression::Prefix{
                token, 
                operator: _, 
                right: _
            } => token.literal.clone(),
            Expression::Infix{
                token, 
                left: _, 
                operator: _, 
                right: _
            } => token.literal.clone(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ident(identifier) => write!(f, "{}", identifier),
            Expression::IntegerLiteral {
                token, 
                value: _,
            } => write!(f, "{}", token.literal),
            Expression::Prefix{
                token: _, 
                operator, 
                right
            } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                token: _,
                left,
                operator,
                right
            } => write!(f, "({} {} {})", left, operator, right),
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

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

