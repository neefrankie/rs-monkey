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
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // 关键字
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone)]
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