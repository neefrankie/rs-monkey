use crate::token;

pub trait Node {
    fn token_literal(&self) -> String;
}

mod statements;
mod expressions;
mod program;

pub use expressions::{
    Identifier,
};

#[derive(Debug)]
pub enum Statement {
    Let {
        token: token::Token,
        name: Identifier,
        value: Box<Expression>,
    },
    Return {
        token: token::Token,
        return_value: Option<Box<Expression>>,
    },
    Expression {
        token: token::Token,
        expression: Box<Expression>,
    },
}


#[derive(Debug)]
pub enum Expression {
    Ident(Identifier),
    IntegerLiteral {
        token: token::Token,
        value: i64,
    },
    Prefix {
        token: token::Token,
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        token: token::Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    Boolean {
        token: token::Token,
        value: bool,
    },
}


pub struct Program {
    pub statements: Vec<Statement>,
}



#[cfg(test)]
mod tests;