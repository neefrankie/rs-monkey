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


#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}