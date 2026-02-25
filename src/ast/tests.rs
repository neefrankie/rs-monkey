use super::*;
use crate::token::{self, TokenType};

#[test]
fn test_string() {
    let program = Program {
        statements: vec![
            Box::new(
                LetStatement {
                    token: token::Token {
                        token_type: TokenType::Let,
                        literal: "let".to_string(),
                    },
                    name: Identifier {
                        token: token::Token {
                            token_type: TokenType::Ident,
                            literal: "myVar".to_string(),
                        },
                        value: "myVar".to_string(),
                    },
                    value: Box::new(
                        Identifier {
                            token: token::Token {
                                token_type: TokenType::Ident,
                                literal: "anotherVar".to_string(),
                            },
                            value: "anotherVar".to_string(),
                        }
                    ),
                },
            )
        ]
    };

    assert_eq!(
        format!("{}", program),
        "let myVar = anotherVar;"
    );
}