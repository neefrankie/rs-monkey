use super::*;
use crate::token::{
    token_from_str,
};

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
    let boolean = new_boolean_expr(true);

    assert_eq!(
        format!("{}", boolean),
        "true"
    );
}

#[test]
fn test_prefix_string() {
    let prefix = new_prefix_expr(
        "!", 
        new_identifier_expr("x")
    );

    assert_eq!(
        format!("{}", prefix),
        "(!x)"
    );
}

#[test]
fn test_infix_string() {
    let infix = new_infix_expr(
        new_identifier_expr("x"),
        "+",
        new_identifier_expr("y")
    );

    assert_eq!(
        format!("{}", infix),
        "(x + y)"
    );
}

#[test]
fn test_if_string() {
    let if_expr = Expression::If {
        token: token_from_str("if"),
        condition: Rc::new(
                new_infix_expr(
                new_identifier_expr("x"),
                "<",
                new_identifier_expr("y")
            )
        ),
        
        consequence: Rc::new(new_block_stmt(vec![
            new_expr_stmt("x", new_identifier_expr("x"))
        ])),
        alternative: Some(Rc::new(new_block_stmt(vec![
            new_expr_stmt("y", new_identifier_expr("y"))
        ])),)
    };

    assert_eq!(
        format!("{}", if_expr),
        "if (x < y) { x } else { y }"
    );
}

#[test]
fn test_function_string() {
    let function = Expression::FunctionLiteral {
        token: token_from_str("fn"),
        parameters: vec![
            new_identifier("x"),
            new_identifier("y")
        ],
        body: Rc::new(new_block_stmt(vec![
            new_expr_stmt(
                "x",
                new_identifier_expr("x")
            )
        ]))
    };

    assert_eq!(
        format!("{}", function),
        "fn(x, y) { x }"
    );
}

#[test]
fn test_call_string() {
    let call = Expression::Call {
        token: token_from_str("add"),
        function: Rc::new(
            new_identifier_expr("add")
        ),
        arguments: vec![
            new_identifier_expr("x"),
            new_identifier_expr("y")
        ]
    };

    assert_eq!(
        format!("{}", call),
        "add(x, y)"
    )
}

#[test]
fn test_let_string() {
    let stmt = new_let_stmt(
        "myVar",
        new_identifier_expr("anotherVar")
    );

    assert_eq!(
        format!("{}", stmt),
        "let myVar = anotherVar;"
    );
}

#[test]
fn test_return_string() {
    let stmt = Statement::Return {
        token: token_from_str("return"),
        return_value: Some(Rc::new(
            new_identifier_expr("x")
        ))
    };

    assert_eq!(
        format!("{}", stmt),
        "return x;"
    );
}

#[test]
fn test_expression_stmt_string() {
    let stmt = Statement::Expression {
        token: token_from_str("x"),
        expression: Rc::new(
            new_identifier_expr("x")
        )
    };

    assert_eq!(
        format!("{}", stmt),
        "x"
    );
}

