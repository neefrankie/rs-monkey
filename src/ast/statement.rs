use std::fmt;

use super::{Statement, BlockStatement};

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let {
                token,
                ..
            } => token.literal.clone(),
            Statement::Return {
                token,
                ..
            } => token.literal.clone(),
            Statement::Expression {
                token,
                ..
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
            } => write!(f, "{} {} = {};", token.literal, name, value),
            Statement::Return {
                token,
                return_value,
            } => {
                write!(f, "{}", token.literal)?;
                if let Some(value) = return_value {
                    write!(f, " {}", value)?;
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


impl BlockStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.statements.iter().map(
            |s| s.to_string()
        ).collect::<Vec<_>>().join("\n"))
    }
}