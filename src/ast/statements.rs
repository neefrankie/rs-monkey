use std::fmt;

use crate::token;
use super::{Node, Statement, Expression};
use super::expressions::Identifier;

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let {
                token,
                name: _,
                value: _,
            } => token.literal.clone(),
            Statement::Return {
                token,
                return_value: _,
            } => token.literal.clone(),
            Statement::Expression {
                token,
                expression: _,
            } => token.literal.clone(),
        }
    }
}


impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let {
                token,
                name,
                value,
            } => write!(f, "{} {} = {}", token.literal, name, value),
            Statement::Return {
                token,
                return_value,
            } => {
                write!(f, "{}", token.literal)?;
                if let Some(value) = return_value {
                    write!(f, " {}", value)?;
                } else {
                    f.write_str("<missing_value>")?;
                }
                write!(f, ";")
            },
            Statement::Expression {
                token: _,
                expression,
            } => write!(f, "{}", expression),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} = {};",
            self.token_literal(), 
            self.name,
            self.value
        )
    }
}


#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Box<Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token_literal())?;

        if let Some(value) = &self.return_value {
            write!(f, " {}", value)?;
        } else {
            f.write_str("<missing_value>")?;
        }

        write!(f, ";")
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Box<Expression>,
}


impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.expression)
    }
}

