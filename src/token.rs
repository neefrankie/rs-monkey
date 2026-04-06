use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TokenType {
    Illegal,
    Eof,

    // 标识符+字面量
    Ident,
    Int,

    // 运算符
    Assign, // =
    Plus, // +
    Minus, // -
    Bang, // !
    Asterisk, // *
    Slash, // /

    LessThan, // <
    GreaterThan, // >

    Eq, // ==
    NotEq, // !=

    // 分隔符
    Comma, // ,
    Semicolon, // ;
    Colon, // :

    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }

    LeftBracket, // [
    RightBracket, // ]

    // 关键字
    Function, // fn
    Let, // let
    True, // true
    False, // false
    If, // if
    Else, // else
    Return, // return

    String, // "string"
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "ILLEGAL"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Ident => write!(f, "IDENT"),
            TokenType::Int => write!(f, "INT"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::LessThan => write!(f, "<"),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::Eq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Function => write!(f, "FUNCTION"),
            TokenType::Let => write!(f, "LET"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::If => write!(f, "IF"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::String => write!(f, "STRING"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub use test_utils::*;