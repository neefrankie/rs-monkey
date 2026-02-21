#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
