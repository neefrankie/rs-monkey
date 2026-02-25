use crate::token::TokenType;

pub const LOWEST: i32 = 0;
pub const EQUAL: i32 = 1; // ==
pub const LESSGRATER: i32 = 2; // > or <
pub const SUM: i32 = 3; // +
pub const PRODUCT: i32 = 4;
pub const PREFIX: i32 = 5; // -X or !X
pub const CALL: i32 = 6; // myFunction(x)

pub fn token_precedence(token_type: TokenType) -> i32 {
    match token_type {
        TokenType::Eq | TokenType::NotEq => EQUAL,
        TokenType::LessThan | TokenType::GreaterThan => LESSGRATER,
        TokenType::Plus | TokenType::Minus => SUM,
        TokenType::Slash | TokenType::Asterisk => PRODUCT,
        _ => LOWEST,
    }
}