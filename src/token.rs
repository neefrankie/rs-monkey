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
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    LBracket,
    RBracket,

    // 关键字
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ident() {
        let tests = vec![
            ("fn", TokenType::Function),
            ("let", TokenType::Let),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("return", TokenType::Return),
            ("my_ident", TokenType::Ident),
        ];

        for (input, expected) in tests {
            let result = lookup_ident(input);
            assert_eq!(result, expected);
        }
    }
}