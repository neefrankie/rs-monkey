use std::fmt;

use crate::token::TokenType;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    Equal = 1, // ==, !=
    LessGreater = 2, // <, >
    Sum = 3, // +, -
    Product = 4, // *, /
    Prefix = 5, // -x, !x
    Call = 6, // a()
    Index = 7, // array[index]
}

impl Precedence {
    pub fn from_token(token_type: TokenType) -> Option<Precedence> {
        match token_type {
            TokenType::Eq |
            TokenType::NotEq => Some(Precedence::Equal),

            TokenType::LessThan |
            TokenType::GreaterThan => Some(Precedence::LessGreater),

            TokenType::Plus |
            TokenType::Minus => Some(Precedence::Sum),

            TokenType::Slash | 
            TokenType::Asterisk => Some(Precedence::Product),

            TokenType::LeftParen => Some(Precedence::Call),

            TokenType::LeftBracket => Some(Precedence::Index),
            
            _ => None,
        }
    }

    // 如果需要获取数值（很少需要）
    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> &'static str {
        match self {
            Precedence::Lowest => "Lowest",
            Precedence::Equal => "Equal",
            Precedence::LessGreater => "LessGreater",
            Precedence::Sum => "Sum",
            Precedence::Product => "Product",
            Precedence::Prefix => "Prefix",
            Precedence::Call => "Call",
            Precedence::Index => "Index",
        }
    }
}

impl fmt::Display for Precedence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.name(), self.value())
    }
}

