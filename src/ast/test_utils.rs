use std::rc::Rc;

use crate::token::{
    new_bool_token, new_int_token, new_string_token, token_from_str
};
use super::{
    Statement,
    BlockStatement,
    Expression,
    Identifier,
};

pub fn new_identifier(value: &str) -> Identifier {
    Identifier {
        token: token_from_str(value),
        value: value.to_string(),
    }
}

pub fn new_identifier_expr(value: &str) -> Expression {
    Expression::Ident(Identifier {
        token: token_from_str(value),
        value: value.to_string(),
    })
}

pub fn new_integer_expr(value: i64) -> Expression {
    Expression::IntegerLiteral {
        token: new_int_token(value),
        value: value,
    }
}

pub fn new_boolean_expr(value: bool) -> Expression {
    Expression::Boolean {
        token: new_bool_token(value),
        value: value,
    }
}

pub fn new_string_expr(value: &str) -> Expression {
    Expression::StringLiteral {
        token: new_string_token(value),
        value: value.to_string(),
    }
}

pub fn new_prefix_expr(operator: &str, right: Expression) -> Expression {
    Expression::Prefix {
        token: token_from_str(operator),
        operator: operator.to_string(),
        right: Rc::new(right),
    }
}

pub fn new_infix_expr(left: Expression, operator: &str, right: Expression) -> Expression {
    Expression::Infix {
        token: token_from_str(operator),
        left: Rc::new(left),
        operator: operator.to_string(),
        right: Rc::new(right),
    }
}



pub fn new_let_stmt(name: &str, value: Expression) -> Statement {
    Statement::Let {
        token: token_from_str("let"),
        name: new_identifier(name),
        value: Rc::new(value),
    }
}

pub fn new_expr_stmt(token_lit: &str, expr: Expression) -> Statement {
    Statement::Expression {
        token: token_from_str(token_lit),
        expression: Rc::new(expr),
    }
}

pub fn new_block_stmt(statements: Vec<Statement>) -> BlockStatement {
    BlockStatement {
        token: token_from_str("{"),
        statements: statements,
    }
}


