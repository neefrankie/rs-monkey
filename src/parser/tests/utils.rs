use crate::token::{Token, TokenType};
use crate::ast::{Expression, Identifier, Program, Statement};

use super::ParseError;

pub fn new_identifier(value: &str) -> Expression {
    Expression::Ident(Identifier {
        token: Token {
            token_type: TokenType::Ident,
            literal: value.to_string(),
        },
        value: value.to_string(),
    })
}

pub fn new_integer(value: i64) -> Expression {
    Expression::IntegerLiteral {
        token: Token {
            token_type: TokenType::Int,
            literal: value.to_string(),
        },
        value: value,
    }
}

pub fn new_boolean(value: bool) -> Expression {
    Expression::Boolean {
        token: Token {
            token_type: if value { TokenType:: True } else { TokenType::False },
            literal: value.to_string(),
        },
        value: value,
    }
}

pub fn unwrap_program(result: Result<Program, Vec<ParseError>>) -> Program {
    match result {
        Ok(program) => program,
        Err(errors) => {
            eprintln!("Parser has {} errors:", errors.len());
            for error in &errors {
                eprint!("Parser error: {:?}", error);
            }
            panic!("Parser has errors");
        },
    }
}

pub fn unwrap_expression_statement(stmt: &Statement) -> &Expression {
    match stmt {
        Statement::Expression {
            expression,
            .. 
        } => expression,
        _ => panic!("Statement is not an ExpressionStatement"),

    }
}