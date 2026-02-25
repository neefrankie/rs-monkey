use std::{any::Any, fmt};

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + fmt::Display + fmt::Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + fmt::Display + fmt::Debug {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        "".to_string()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

mod statements;
mod expressions;

pub use statements::{
    LetStatement, 
    ReturnStatement, 
    ExpressionStatement
};
pub use expressions::{
    MissingExpression,
    Identifier,
    IntegerLiteral, 
    PrefixExpression, 
    InfixExpression, 
};


#[cfg(test)]
mod tests;