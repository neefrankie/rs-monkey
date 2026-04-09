use super::*;

pub fn token_from_str(s: &str) -> Token {
    match s {
        "=" => Token {
            token_type: TokenType::Assign,
            literal: s.to_string(),
        },
        "+" => Token {
            token_type: TokenType::Plus,
            literal: s.to_string(),
        },
        "-" => Token {
            token_type: TokenType::Minus,
            literal: s.to_string(),
        },
        "!" => Token {
            token_type: TokenType::Bang,
            literal: s.to_string(),
        },
        "*" => Token {
            token_type: TokenType::Asterisk,
            literal: s.to_string(),
        },
        "/" => Token {
            token_type: TokenType::Slash,
            literal: s.to_string(),
        },
        "<" => Token {
            token_type: TokenType::LessThan,
            literal: s.to_string(),
        },
        ">" => Token {
            token_type: TokenType::GreaterThan,
            literal: s.to_string(),
        },
        "==" => Token {
            token_type: TokenType::Eq,
            literal: s.to_string(),
        },
        "!=" => Token {
            token_type: TokenType::NotEq,
            literal: s.to_string(),
        },
        "," => Token {
            token_type: TokenType::Comma,
            literal: s.to_string(),
        },
        ";" => Token {
            token_type: TokenType::Semicolon,
            literal: s.to_string(),
        },
        ":" => Token {
            token_type: TokenType::Colon,
            literal: s.to_string(),
        },
        "(" => Token {
            token_type: TokenType::LeftParen,
            literal: s.to_string(),
        },
        ")" => Token {
            token_type: TokenType::RightParen,
            literal: s.to_string(),
        },
        "{" => Token {
            token_type: TokenType::LeftBrace,
            literal: s.to_string(),
        },
        "}" => Token {
            token_type: TokenType::RightBrace,
            literal: s.to_string(),
        },
        "[" => Token {
            token_type: TokenType::LeftBracket,
            literal: s.to_string(),
        },
        "]" => Token {
            token_type: TokenType::RightBracket,
            literal: s.to_string(),
        },
        "fn" => new_fn_token(),
        "let" => new_let_token(),
        "true" => new_bool_token(true),
        "false" => new_bool_token(false),
        "if" => new_if_token(),
        "else" => new_else_token(),
        "return" => new_return_token(),
        "EOF" => new_eof_token(),
        _ => {
            if s.chars().all(|c| c.is_ascii_digit()) {
                Token {
                    token_type: TokenType::Int,
                    literal: s.to_string(),
                }
            } else if is_letter(s) {
                Token {
                    token_type: TokenType::Ident,
                    literal: s.to_string(),
                }
            } else {
                Token {
                    token_type: TokenType::Illegal,
                    literal: s.to_string(),
                }
            }
        }
    }
}

fn is_letter(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
}

pub fn new_eof_token() -> Token {
    Token {
        token_type: TokenType::Eof,
        literal: "".to_string(),
    }
}

pub fn new_int_token(value: i64) -> Token {
    Token {
        token_type: TokenType::Int,
        literal: value.to_string(),
    }
}

pub fn new_string_token(value: &str) -> Token {
    Token {
        token_type: TokenType::String,
        literal: value.to_string(),
    }
}

pub fn new_ident_token(value: &str) -> Token {
    match value {
        "fn" => new_fn_token(),
        "let" => new_let_token(),
        "true" => new_bool_token(true),
        "false" => new_bool_token(false),
        "if" => new_if_token(),
        "else" => new_else_token(),
        "return" => new_return_token(),
        _ => Token {
            token_type: TokenType::Ident,
            literal: value.to_string(),
        },
    }
}

pub fn new_bool_token(value: bool) -> Token {
    Token {
        token_type: if value {
                TokenType::True
            } else {
                TokenType::False
            },
        literal: if value { 
                "true".to_string()
            } else { 
                "false".to_string()
            },
    }
}

pub fn new_fn_token() -> Token {
    Token {
        token_type: TokenType::Function,
        literal: "fn".to_string(),
    }
}

pub fn new_let_token() -> Token {
    Token {
        token_type: TokenType::Let,
        literal: "let".to_string(),
    }
}

pub fn new_if_token() -> Token {
    Token {
        token_type: TokenType::If,
        literal: "if".to_string(),
    }
}

pub fn new_else_token() -> Token {
    Token {
        token_type: TokenType::Else,
        literal: "else".to_string(),
    }
}

pub fn new_return_token() -> Token {
    Token {
        token_type: TokenType::Return,
        literal: "return".to_string(),
    }
}

