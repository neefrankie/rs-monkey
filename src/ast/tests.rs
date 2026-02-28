use super::*;
use crate::token::{self, Token, TokenType};

fn new_int_token(value: i64) -> Token {
    Token {
        token_type: TokenType::Int,
        literal: value.to_string(),
    }
}

fn new_bool_token(value: bool) -> Token {
    Token {
        token_type: if value { TokenType::True} else { TokenType::False},
        literal: value.to_string(),
    }
}

fn new_prefix_token(operator: &str) -> Token {
    let token_type = match operator {
        "!" => TokenType::Bang,
        "-" => TokenType::Minus,
        _ => panic!("Invalid operator"),
    };
    Token {
        token_type,
        literal: operator.to_string(),
    }
}

fn new_operator_token(operator: &str) -> Token {
    let token_type = match operator {
        "+" => TokenType::Plus,
        "-" => TokenType::Minus,
        "*" => TokenType::Asterisk,
        "/" => TokenType::Slash,
        "==" => TokenType::Eq,
        "!=" => TokenType::NotEq,
        "<" => TokenType::LessThan,
        ">" => TokenType::GreaterThan,
        _ => panic!("Invalid operator"),
    };

    return Token {
        token_type,
        literal: operator.to_string(),
    };
}

fn new_if_token() -> Token {
    Token {
        token_type: TokenType::If,
        literal: "if".to_string(),
    }
}

fn new_let_token() -> Token {
    Token {
        token_type: TokenType::Let,
        literal: "let".to_string(),
    }
}

fn new_ident_token(value: &str) -> Token {
    Token {
        token_type: TokenType::Ident,
        literal: value.to_string(),
    }
}

fn new_left_brace_token() -> Token {
    Token {
        token_type: TokenType::RBrace,
        literal: "{".to_string(),
    }
}

fn new_function_token() -> Token {
    Token {
        token_type: TokenType::Function,
        literal: "fn".to_string(),
    }
}

fn new_identifier(value: &str) -> Identifier {
    Identifier {
        token: new_ident_token(value),
        value: value.to_string(),
    }
}

fn new_ident_expr(value: &str) -> Expression {
    Expression::Ident(
        new_identifier(value)
    )
}

fn new_integer_expr(value: i64) -> Expression {
    Expression::IntegerLiteral {
        token: new_int_token(value),
        value: value,
    }
}

fn new_bool_expr(value: bool) -> Expression {
    Expression::Boolean {
        token: new_bool_token(value),
        value: value,
    }
}

fn new_prefix_expr(operator: &str, right: Expression) -> Expression {
    Expression::Prefix {
        token: new_prefix_token(operator),
        operator: operator.to_string(),
        right: Box::new(right),
    }
}

fn new_infix_expr(left: Expression, operator: &str, right: Expression) -> Expression {
    Expression::Infix {
        token: new_operator_token(operator),
        left: Box::new(left),
        operator: operator.to_string(),
        right: Box::new(right),
    }
}

fn new_block_stmt(statements: Vec<Statement>) -> BlockStatement {
    BlockStatement {
        token: new_left_brace_token(),
        statements: statements,
    }
}


fn new_let_stmt(name: &str, value: Expression) -> Statement {
    Statement::Let {
        token: new_let_token(),
        name: new_identifier(name),
        value: Box::new(value),
    }
}

#[test]
fn test_identifier_string() {
    let ident = new_identifier("x");

    assert_eq!(
        format!("{}", ident),
        "x"
    );
}

#[test]
fn test_integer_string() {
    let integer = new_integer_expr(5);

    assert_eq!(
        format!("{}", integer),
        "5"
    );
}

#[test]
fn test_boolean_string() {
    let boolean = new_bool_expr(true);

    assert_eq!(
        format!("{}", boolean),
        "true"
    );
}

#[test]
fn test_prefix_string() {
    let prefix = new_prefix_expr(
        "!", 
        new_ident_expr("x")
    );

    assert_eq!(
        format!("{}", prefix),
        "(!x)"
    );
}

#[test]
fn test_infix_string() {
    let infix = new_infix_expr(
        new_ident_expr("x"),
        "+",
        new_ident_expr("y")
    );

    assert_eq!(
        format!("{}", infix),
        "(x + y)"
    );
}

#[test]
fn test_if_string() {
    let if_expr = Expression::If {
        token: new_if_token(),
        condition: Box::new(
                new_infix_expr(
                new_ident_expr("x"),
                "<",
                new_ident_expr("y")
            )
        ),
        consequence: new_block_stmt(vec![
            Statement::Expression {
                token: new_ident_token("x"),
                expression: Box::new(
                    new_ident_expr("x")
                )
            }
        ]),
        alternative: Some(new_block_stmt(vec![
            Statement::Expression {
                token: new_ident_token("y"),
                expression: Box::new(
                    new_ident_expr("y")
                )
            }
        ]),)
    };

    assert_eq!(
        format!("{}", if_expr),
        "if (x < y) { x } else { y }"
    );
}

#[test]
fn test_let_string() {
    let stmt = new_let_stmt(
        "myVar",
        new_ident_expr("anotherVar")
    );

    assert_eq!(
        format!("{}", stmt),
        "let myVar = anotherVar;"
    );
}

