use crate::token::{Token, TokenType};
use crate::ast::{Expression, Identifier, Program, Statement, Node};

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

pub fn assert_identifier_expression(expr: &Expression, expected: &str) {

    let Expression::Ident(ident) = expr else {
        panic!("Expected Identifier, got {:?}", expr);
    };

    assert_identifier(ident, expected);
}

pub fn assert_identifier(ident: &Identifier, expected: &str) {
    assert_eq!(
        ident.value,
        expected,
        "Identifier.value not {}. got={}",
        expected,
        ident.value
    );

    assert_eq!(
        ident.token_literal(),
        expected,
        "Identifier.token_literal not {}. got={}",
        expected,
        ident.token_literal()
    );
}

pub fn assert_integer_literal(expr: &Expression, expected: i64) {
    let Expression::IntegerLiteral { value , ..} = expr else {
        panic!("Expected IntegerLiteral, got {}", expr);
    };

    assert_eq!(
        *value,
        expected,
        "IntegralLiteral.value is not {}. got={}",
        expected,
        *value,
    );
    assert_eq!(
        expr.token_literal(),
        expected.to_string(),
        "expr.token_literal is not {}. got={}",
        expected,
        expr.token_literal()
    );
}

pub fn assert_boolean(expr: &Expression, expected: bool) {
    match expr {
        Expression::Boolean { value, ..} => {
            assert_eq!(
                *value,
                expected,
                "Boolean.value is not {}. got={}",
                expected,
                *value,
            );
            assert_eq!(
                expr.token_literal(),
                expected.to_string(),
                "expr.token_literal is not {}. got={}",
                expected,
                expr.token_literal()
            )
        }
        _ => panic!("Expected Boolean, got {}", expr),
    }
}

pub fn assert_string(expr: &Expression, expected: &str) {
    let Expression::StringLiteral { value, .. } = expr else {
        panic!("exp not ast.StringLiteral. got={}", expr)
    };

    assert_eq!(
        *value,
        expected,
        "StringLiteral.value is not {}. got={}",
        expected,
        *value,
    );
}

pub fn assert_prefix_expression(
    expr: &Expression, 
    expected_operator: &str, 
    expected_right: &Expression
) {
    let Expression::Prefix { operator, right , ..} = expr else {
        panic!("exp not Expression::Prefix. got={}", expr)
    };

    assert_eq!(
        *operator,
        expected_operator,
        "exp.Operator is not {}. got={}",
        expected_operator,
        operator
    );

    assert_literal_expression(
        right, 
        expected_right
    );
}

fn assert_literal_expression(expr: &Expression, expected: &Expression) {
    match expected {
        Expression::Ident(identifier) => {
            assert_identifier_expression(
                expr, 
                &identifier.value)
        }
        Expression::IntegerLiteral { value, .. } => {
            assert_integer_literal(
                expr, 
                *value
            );
        }
        
        Expression::Boolean { value , ..} => {
            assert_boolean(
                expr, 
                *value
            );
        }
        _ => {
            panic!("type of exp not handled. got {}", expr);
        }
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


pub fn assert_statements_len(program: &Program, expected: usize) {
    assert_eq!(
        program.statements.len(),
        expected,
        "program.statements does not contain {} statement. got={}", expected,
        program.statements.len()
    );
}










pub fn assert_infix_expression(
    exp: &Expression, 
    left: &Expression, 
    operator: String, 
    right: &Expression
) {
    let Some((
        infix_left, 
        infix_operator, 
        infix_right
    )) = exp.as_infix() else {
        panic!("expression is not an Expression::Infix. got {}", exp);
    };

    assert_literal_expression(
        infix_left, 
        left
    );
    
    assert_eq!(
        infix_operator,
        operator,
        "exp.operator is not {}. got={}",
        operator,
        infix_operator
    );

    assert_literal_expression(
        infix_right,
        right
    );
}

pub fn assert_let_statement(stmt: &Statement, expected_name: &str) {

    assert_eq!(
        stmt.token_literal(),
        "let",
        "stmt.token_literal() is not 'let'. got={}",
        stmt.token_literal(),
    );

    match stmt {
        Statement::Let { 
            name, 
            ..
        } => {
            assert_eq!(
                name.value,
                expected_name,
                "Statement.Let.name.value not '{}'. got '{}'",
                expected_name,
                name.value
            );

            assert_eq!(
                name.token_literal(),
                expected_name,
                "Statement.Let.name.token_literal() not '{}'. got '{}'",
                expected_name,
                name.token_literal()
            );
        }

        _ => panic!("stmt is not a LetStatement"),
    }
}

