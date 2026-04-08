use std::rc::Rc;

use crate::ast::{
    Expression,
    Identifier,
    Program,
    Statement,
    BlockStatement,
};

use super::ParseError;

// fn assert_expression(expr: &Expression, expected: &Expression) {
//     match expr {
//         Expression::Ident(ident) => {
//             assert_identifier(ident, expected);
//         }
//     }
// }

pub fn assert_identifier_expr(expr: &Expression, expected: &str) {

    let Expression::Ident(ident) = expr else {
        panic!("Expected Identifier, got {:?}", expr);
    };

    assert_identifier_value(ident, expected);
}

pub fn assert_identifier_value(ident: &Identifier, expected: &str) {
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

    assert_expr(
        right, 
        expected_right
    );
}

pub fn assert_infix_expression(
    expr: &Expression,
    expected_left: &Expression,
    expected_operator: &str,
    expected_right: &Expression
) {
    let Expression::Infix { 
        left, 
        operator, 
        right, 
        ..
    } = expr else {
        panic!("exp not Expression::Infix. got={}", expr)
    };

    assert_eq!(
        *operator,
        expected_operator,
        "exp.operator is not {}. got={}",
        expected_operator,
        operator
    );

    assert_expr(
        left, 
        expected_left
    );

    assert_expr(
        right, 
        expected_right
    );
}

pub fn assert_if_expression(
    expr: &Expression,
    expected_condition: &Expression,
    expected_consequence: &BlockStatement,
    expected_alternative: Option<&BlockStatement>,
) {
    let Expression::If {
        condition,
        consequence,
        alternative,
        ..
    } = expr
    else {
        panic!("Not an Express::If. got={}", expr);
    };

    assert_expr(condition, expected_condition);

    assert_block_stmt(consequence, expected_consequence);

    assert_optional_block_stmt(alternative, expected_alternative);
}

pub fn assert_function_literal(
    expr: &Expression,
    expected_parameters: Vec<Identifier>,
    expected_body: &BlockStatement,
) {
    let Expression::FunctionLiteral {
        parameters,
        body,
        ..
    } = expr
    else {
        panic!("Not an Express::FunctionLiteral. got={}", expr);
    };

    assert_eq!(parameters.len(), expected_parameters.len());

    for (i, param) in parameters.iter().enumerate() {
        assert_identifier_value(
            param, 
            &expected_parameters[i].value
        );
    }

    assert_block_stmt(
        body,
        expected_body,
    );
}

pub fn assert_call_expression(
    expr: &Expression, 
    expected_func: &Expression, 
    expected_args: Vec<Expression>
) {
    let Expression::Call {
        function,
        arguments,
        ..
    } = expr else {
        panic!("Not an Expression::Call")
    };

    assert_expr(function, expected_func);
    assert_eq!(arguments.len(), expected_args.len());

    for (i, arg) in arguments.iter().enumerate() {
        assert_expr(arg, &expected_args[i]);
    }
}

pub fn assert_expr(expr: &Expression, expected: &Expression) {
    match expected {
        Expression::Ident(identifier) => {
            assert_identifier_expr(
                expr, 
                &identifier.value)
        },
        Expression::IntegerLiteral { value, .. } => {
            assert_integer_literal(
                expr, 
                *value
            );
        },
        
        Expression::Boolean { value , ..} => {
            assert_boolean(
                expr, 
                *value
            );
        },
        Expression::Infix { 
            left, 
            operator, 
            right,
            ..
        } => {
            assert_infix_expression(
                expr, 
                left, 
                operator, 
                right
            );
        },
        Expression::If {
            condition,
            consequence,
            alternative,
            ..
        } => {
            assert_if_expression(
                expr,
                condition,
                consequence,
                alternative.as_deref(),
            );
        },
        _ => {
            panic!("type of exp not handled. got {}", expr);
        },
    }
}


fn assert_optional_expr(
    expr: &Option<Rc<Expression>>,
    expected: Option<&Expression>
) {
    match (expr, expected) {
        (Some(expr), Some(expected)) => {
            assert_expr(expr, expected);
        },
        (None, None) => {},
        _ => panic!("expression and expected expression do not match"),
    }
}


pub fn assert_block_stmt(
    stmt: &BlockStatement,
    expected: &BlockStatement
) {
    assert_eq!(stmt.statements.len(), expected.statements.len());

    for (i, stmt) in stmt.statements.iter().enumerate() {
        assert_stmt(stmt, &expected.statements[i]);
    }
}

fn assert_optional_block_stmt(
    stmt: &Option<Rc<BlockStatement>>,
    expected: Option<&BlockStatement>
) {
    match (stmt, expected) {
        (Some(blocks), Some(expected)) => assert_block_stmt(blocks, expected),
        (None, None) => {}
        _ => panic!("block statement not equal"),
    }
}

pub fn assert_stmt(stmt: &Statement, expected: &Statement) {
    match expected {
        Statement::Let { 
            name,
            value,
            ..
        } => {
            assert_let_stmt(&stmt, name, value);
        },
        Statement::Return {
            return_value,
            ..
        } => {
            assert_return_stmt(&stmt, return_value.as_deref());
        },
        Statement::Expression { 
            expression,
            ..
        } => {
            assert_expr_stmt(&stmt, expression);
        }
    }
}

pub fn assert_let_stmt(
    stmt: &Statement, 
    expected_name: &Identifier, 
    expected_value: &Expression
) {
    let Statement::Let {
        name,
        value,
        ..
    } = stmt else {
        panic!("stmt is not a LetStatement")
    };

    assert_identifier_value(name, &expected_name.value);

    assert_expr(value, expected_value);
}

pub fn assert_return_stmt(
    stmt: &Statement, 
    expected_value: Option<&Expression>
) {
    let Statement::Return {
        return_value,
        ..
    } = stmt else {
        panic!("stmt is not a ReturnStatement")
    };

    assert_optional_expr(
        return_value, 
        expected_value
    );
}

pub fn assert_expr_stmt(stmt: &Statement, expected_value: &Expression) {
    let Statement::Expression {
        expression,
        ..
    } = stmt else {
        panic!("stmt is not a ExpressionStatement")
    };

    assert_expr(expression, expected_value);
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

