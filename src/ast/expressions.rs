use std::fmt;

use super::{Node, Expression, Identifier, BlockStatement};

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

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Ident(identifier) => identifier.token_literal(),

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

            Expression::If { 
                token,
                ..
            } => token.literal.clone(),

            Expression::FunctionLiteral { 
                token,
                .. 
            } => token.literal.clone(),

            Expression::Call { 
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

            Expression::If {
                condition,
                consequence,
                alternative,
                ..
            } => {
                write!(
                    f,
                    "if {} {{ {} }}",
                    condition,
                    consequence
                )?;

                match alternative {
                    Some(alt) => {
                        write!(f, " else {{ {} }}", alt)
                    },
                    None => Ok(()),
                }
            },

            Expression::FunctionLiteral {
                token,
                parameters,
                body 
            } => {
                let params = parameters.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f, 
                    "{} ({}) {}", 
                    token.literal, 
                    params, 
                    body
                )
            },

            Expression::Call {
                function,
                arguments,
                ..
            } => {
                let args = arguments.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f, 
                    "{}({})", 
                    function, 
                    args
                )
            },
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

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Expression::Boolean {
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

    pub fn as_if(&self) -> Option<(
        &Expression, 
        &BlockStatement, 
        Option<&BlockStatement>
    )> {
        match self {
            Expression::If {
                condition,
                consequence,
                alternative,
                ..
            } => Some((
                condition.as_ref(),
                consequence,
                alternative.as_ref(),
            )),
            _ => None,
        }
    }
}




