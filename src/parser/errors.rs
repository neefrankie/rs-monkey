use std::{error, fmt};

use crate::token::TokenType;


#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: TokenType,
        got: TokenType,
    },
    InvalidSyntax {
        message: String,
    },
    ExpectedInt {
        got: String,
    },
    NoPrefixParseFn {
        token_type: TokenType,
    },
    NoInfixParseFn {
        token_type: TokenType,
    },
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, got } => {
                write!(
                    f,
                    "Unexpected token. Expected {}, got {}",
                    expected, got
                )
            }
            ParseError::InvalidSyntax { message } => {
                write!(f, "Invalid syntax: {}", message)
            }
            ParseError::ExpectedInt { got } => {
                write!(f, "Cound not parse {} as integer", got)
            }
            ParseError::NoPrefixParseFn { token_type } => {
                write!(f, "No prefix parse function for {}", token_type)
            }
            ParseError::NoInfixParseFn { token_type } => {
                write!(f, "No infix parse function for {}", token_type)
            }
        }
    }

}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Position {
//     pub line: usize,
//     pub column: usize,
// }

impl error::Error for ParseError {
    
}