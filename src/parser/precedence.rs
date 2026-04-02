use crate::token::TokenType;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    Equal = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
    Index = 7,
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

            TokenType::LParen => Some(Precedence::Call),
            
            _ => None,
        }
    }

    // 如果需要获取数值（很少需要）
    // pub fn value(&self) -> u8 {
    //     *self as u8
    // }
}

// 如果你有很多优先级，可以用宏避免手动赋值：
// macro_rules! define_precedence {
//     ($($name:ident),* $(,)?) => {
//         #[repr(u8)]
//         #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
//         pub enum Precedence {
//             $($name,)*
//         }
        
//         impl Precedence {
//             $(
//                 pub const $name: Precedence = Precedence::$name;
//             )*
//         }
//     };
// }

// // 使用
// define_precedence! {
//     Lowest,
//     Equals,
//     LessGreater, 
//     Sum,
//     Product,
//     Prefix,
//     Call,
// }