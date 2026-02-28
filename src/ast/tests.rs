use super::*;
use crate::token::{self, Token, TokenType};

#[test]
fn test_string() {
    let program = Program {
        statements: vec![
            Statement::Let {
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
                value: Box::new(Expression::Ident(
                    Identifier {
                        token: token::Token {
                            token_type: TokenType::Ident,
                            literal: "anotherVar".to_string(),
                        },
                        value: "anotherVar".to_string(),
                    }
                )),
            },
        ]
    };

    assert_eq!(
        format!("{}", program),
        "let myVar = anotherVar;"
    );
}

#[test]
fn test_if_string() {
    let input = r#"if (x < y) { x } else { y }"#;
    let if_expr = Expression::If {
        token: token::Token {
            token_type: TokenType::If,
            literal: "if".to_string(),
        },
        condition: Box::new(Expression::Infix {
            token: Token {
                token_type: TokenType::LessThan,
                literal: "<".to_string(),
            },
            left: Box::new(Expression::Ident(Identifier {
                token: Token {
                    token_type: TokenType::Ident,
                    literal: "x".to_string(),
                },
                value: "x".to_string(),
            })),
            operator: "<".to_string(),
            right: Box::new(Expression::Ident(Identifier {
                token: Token {
                    token_type: TokenType::Ident,
                    literal: "y".to_string()
                },
                value: "y".to_string(),
            })),
        }),
        consequence: BlockStatement {
            token: token::Token {
                token_type: TokenType::LBrace,
                literal: "{".to_string(),
            },
            statements: vec![Statement::Expression {
                token: token::Token {
                    token_type: TokenType::Ident,
                    literal: "x".to_string(),
                },
                expression: Box::new(Expression::Ident(
                    Identifier {
                        token: token::Token {
                            token_type: TokenType::Ident,
                            literal: "x".to_string(),
                        },
                        value: "x".to_string(),
                    }
                ))
            }]
        },
        alternative: Some(BlockStatement {
            token: token::Token {
                token_type: TokenType::LBrace,
                literal: "{".to_string(),
            },
            statements: vec![Statement::Expression {
                token: token::Token {
                    token_type: TokenType::Ident,
                    literal: "y".to_string(),
                },
                expression: Box::new(Expression::Ident(
                    Identifier {
                        token: token::Token {
                            token_type: TokenType::Ident,
                            literal: "y".to_string(),
                        },
                        value: "y".to_string(),
                    }
                ))
            }]
        })
    };

    assert_eq!(
        format!("{}", if_expr),
        input
    );
}