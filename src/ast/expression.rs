use std::fmt;

use super::{Node, Expression, Identifier};

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

            Expression::StringLiteral {
                token,
                ..
            } => token.literal.clone(),

            Expression::ArrayLiteral {
                token,
                ..
            } => token.literal.clone(),

            Expression::Index {
                token,
                ..
            } => token.literal.clone(),

            Expression::HashLiteral {
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

            Expression::Boolean {
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
                    "{}({}) {{ {} }}", 
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

            Expression::StringLiteral {
                token,
                ..
            } => write!(f, "\"{}\"", token.literal),

            Expression::ArrayLiteral {
                elements,
                ..
            } => {
                let elements = elements.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "[{}]",
                    elements
                )
            },

            Expression::Index { left, index, .. } => {
                write!(f, "({}[{}])", left.to_string(), index.to_string())
            },

            Expression::HashLiteral {
                pairs,
                ..
            } => {
                let pairs_vec = pairs.iter()
                    .map(|(key, value)| {
                        format!("{}: {}", key.to_string(), value.to_string())
                    })
                    .collect::<Vec<_>>();

                write!(f, "{{{}}}", pairs_vec.join(", "))
            }
        }
    }
}





