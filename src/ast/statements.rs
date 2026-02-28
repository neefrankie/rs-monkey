use std::fmt;

use super::{Node, Statement, Expression, Identifier, BlockStatement};

impl Node for Statement {
    fn token_literal(&self) -> String {
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

impl Statement {
    pub fn as_expression(&self) -> Option<&Expression> {
        match self {
            Statement::Expression {
                token: _,
                expression,
            } => Some(expression),
            _ => None,
        }
    }

    pub fn as_let(&self) -> Option<(&Identifier, &Expression)> {
        match self {
            Statement::Let {
                name,
                value,
                ..
            } => Some((name, value.as_ref())),
            _ => None,
        }
    }

    pub fn as_return(&self) -> Option<&Expression> {
        match self {
            Statement::Return {
                return_value,
                ..
            } => return_value.as_deref(),
            _ => None,
        }
    }

    pub fn is_expression(&self) -> bool {
        matches!(self, Statement::Expression { .. })
    }

    pub fn is_let(&self) -> bool {
        matches!(self, Statement::Let { .. })
    }
}


impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.statements.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n"))
    }
}