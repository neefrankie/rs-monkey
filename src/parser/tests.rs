use crate::token::{Token, TokenType};
use crate::ast::{Expression, Identifier, Node, Program, Statement};
use crate::lexer::{Lexer};

use super::precedence::{Precedence};
use super::Parser;
use super::errors::ParseError;

fn assert_identifier_expression(expr: &Expression, expected: &str) {
    match expr {
        Expression::Ident(ident) => {
            assert_identifier(ident, expected);
        }
        _ => panic!("Expected Identifier, got {:?}", expr),
    }
}

fn assert_identifier(ident: &Identifier, expected: &str) {
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

fn assert_integer_literal(expr: &Expression, expected: i64) {
    match expr {
        Expression::IntegerLiteral { value, ..} => {
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

        _ => panic!("Expected IntegerLiteral, got {}", expr),
    }
}

fn assert_boolean(expr: &Expression, expected: bool) {
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

fn assert_prefix_expression(
    expr: &Expression, 
    expected_operator: &str, 
    expected_right: &Expression
) {
    match expr {
        Expression::Prefix { 
            operator,
            right,
            ..
        } => {
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

        _ => panic!("Expected Prefix expression. got {}", expr),
    }
}

fn assert_infix_expression(
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

fn assert_let_statement(stmt: &Statement, expected_name: &str) {

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

fn assert_no_parse_errors(result: Result<Program, Vec<ParseError>>) -> Program {
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


#[test]
fn test_precedence() {
    assert!(Precedence::Lowest < Precedence::Equal);
    assert!(Precedence::Equal < Precedence::LessGreater);
    assert!(Precedence::LessGreater < Precedence::Sum);
    assert!(Precedence::Sum < Precedence::Product);
    assert!(Precedence::Product < Precedence::Prefix);
    assert!(Precedence::Prefix < Precedence::Call);
}

#[test]
fn test_precedence_from_token() {
    assert_eq!(
        Precedence::from_token(TokenType::Plus),
        Some(Precedence::Sum)
    );
    assert_eq!(
        Precedence::from_token(TokenType::Asterisk), 
        Some(Precedence::Product)
    );
    assert_eq!(
        Precedence::from_token(TokenType::LBrace),
        None
    );
}


#[test]
fn test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;
";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let programm = assert_no_parse_errors(parser.parse_program());

    assert_eq!(
        programm.statements.len(), 
        3,
        "program.statements does not contain 3 statements. got={}",
        programm.statements.len());

    let tests = vec![
        "x",
        "y",
        "foobar",
    ];

    for (i, &expected_name) in tests.iter().enumerate() {
        // &* 先解引用得到 dyn Statement，再取引用得到 &dyn Statement
        let stmt = &programm.statements[i];

        assert_let_statement(stmt, expected_name);
    }
}

#[test]
fn test_return_statements() {
    let input = "return 5;
return 10;
return 993 322;
";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());
    
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    for stmt in program.statements {
        assert_eq!(
            stmt.token_literal(),
            "return",
            "returnStmt.token_literal not 'return', got {}",
            stmt.token_literal()
        )
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());

    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got={}",
        program.statements.len()
    );

    let expr = program.statements[0]
        .as_expression()
        .expect("program.statements[0] is not an ExpressionStatement");

    assert_identifier_expression(expr, "foobar");
}


#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());

    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got={}",
        program.statements.len()
    );
 
    let expr = program.statements[0]
        .as_expression()
        .expect("program.statements[0] is not ExpressionStatement");

    assert_integer_literal(expr, 5);
}

#[test]
fn test_parsing_prefix_expressions() {
    let five = Expression::IntegerLiteral {
         token: Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
         }, 
         value: 5,
    };

    let fifteen = Expression::IntegerLiteral {
         token: Token {
            token_type: TokenType::Int,
            literal: "15".to_string(),
         }, 
         value: 15,
    };

    let bool_true = Expression::Boolean {
         token: Token {
            token_type: TokenType::True,
            literal: "true".to_string(),
         }, 
         value: true,
    };

    let bool_false = Expression::Boolean {
         token: Token {
            token_type: TokenType::False,
            literal: "false".to_string(),
         }, 
         value: false,
    };

    let tests = vec![
        ("!5;", "!", &five),
        ("-15;", "-", &fifteen),
        ("!true;", "!", &bool_true),
        ("!false;", "!", &bool_false),
    ];

    for (
        input, 
        expected_operator, 
        expected_right
    ) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = assert_no_parse_errors(parser.parse_program());

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        let expr = program.statements[0]
            .as_expression()
            .expect("program.statements[0] is not ExpressionStatement");

        assert_prefix_expression(
            expr, 
            expected_operator, 
            expected_right
        );
    }
}


#[test]
fn test_parsing_infix_expressions() {
    let five = Expression::IntegerLiteral {
         token: Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
         }, 
         value: 5,
    };

    let bool_true = Expression::Boolean {
         token: Token {
            token_type: TokenType::True,
            literal: "true".to_string(),
         }, 
         value: true,
    };

    let bool_false = Expression::Boolean {
         token: Token {
            token_type: TokenType::False,
            literal: "false".to_string(),
         }, 
         value: false,
    };

    let tests = vec![
        ("5 + 5;", &five, "+", &five),
        ("5 - 5;", &five, "-", &five),
        ("5 * 5;", &five, "*", &five),
        ("5 / 5;", &five, "/", &five),
        ("5 > 5;", &five, ">", &five),
        ("5 < 5;", &five, "<", &five),
        ("true == true", &bool_true, "==", &bool_true),
        ("true != false", &bool_true, "!=", &bool_false),
        ("false == false", &bool_false, "==", &bool_false),
    ];

    for (
        input, 
        left_value, 
        expected_operator, 
        right_value
    ) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = assert_no_parse_errors(parser.parse_program());
        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        let expr = program.statements[0]
            .as_expression()
            .expect("statement is not an ExpressionStatement");

        assert_infix_expression(
            expr, 
            left_value, 
            expected_operator.to_string(), 
            right_value
        );
    }
}


#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "(a + add((b * c)) + d)"),
        ("add(a, b, 1, 2 * 3, 4 + 5", "add(6, 7 * 8)"),
        ("add(a + b + c * d / f + g", "add((((a + b) + (c * d) / f)) + g)")

    ];

    for (input, expected) in tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = assert_no_parse_errors(p.parse_program());

        let actual = program.to_string();
        assert_eq!(
            actual, 
            expected,
            "Expected: {}, got: {}",
            expected,
            actual
        );
    }
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());

    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );
    // program.statements[0].expression
    let expr = program.statements[0]
        .as_expression()
        .expect("program.statements[0] is not ExpressionStatement");

    let (
        condition, 
        consequence, 
        alternative
    ) = expr.as_if()
        .expect("expr is not an IfExpression");

    assert_infix_expression(
        condition,
        &Expression::Ident(Identifier {
            token: Token {
                token_type: TokenType::Ident,
                literal: "x".to_string(),
            },
            value: "x".to_string(),
        }), 
        "<".to_string(), 
        &Expression::Ident(Identifier {
            token: Token {
                token_type: TokenType::Ident,
                literal: "y".to_string(),
            },
            value: "y".to_string(),
        }),
    );

    assert_eq!(
        consequence.statements.len(),
        1,
        "consequence.statements does not contain 1 statement. got={}",
        consequence.statements.len()
    );

    let ident = consequence.statements[0]
        .as_expression()
        .expect("consequence.statements[0] is not ExpressionStatement");

    assert_identifier_expression(ident, "x");

    assert!(alternative.is_none());
}

#[test]
fn test_function_literal_parsing() {
    let input = "fn(x, y) { x + y; }";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    let expr = program.statements[0]
        .as_expression()
        .expect("program.statements[0] is not ExpressionStatement");

    match expr {
        Expression::FunctionLiteral {
            parameters,
            body,
            ..
        } => {
            assert_eq!(
                parameters.len(),
                2,
                "function literal parameters wrong. want 2, got={}",
                parameters.len()
            );

            assert_identifier(&parameters[0], "x");
            assert_identifier(&parameters[1], "y");

            assert_eq!(
                body.statements.len(),
                1,
                "function.Body.statements does not contain 1 statement. got={}",
                body.statements.len(),
            );

            match &body.statements[0] {
                Statement::Expression { 
                    expression,
                    ..
                } => {
                    assert_infix_expression(
                        &*expression, 
                        &Expression::Ident(Identifier {
                             token: Token { 
                                token_type: TokenType::Ident, 
                                literal: "x".to_string() 
                            }, 
                             value: "x".to_string(),
                        }), 
                        "+".to_string(), 
                        &Expression::Ident(Identifier {
                             token: Token { 
                                token_type: TokenType::Ident, 
                                literal: "y".to_string() 
                            }, 
                             value: "y".to_string(),
                        }),
                    );
                }

                _ => panic!("function body stmt is not ExpressionStatement. got={}", body.statements[0]),
            }
        }

        _ => panic!("stmt.Expression is not a FunctionLiteral. got={}", expr),
    }
}

#[test]
fn test_function_parameter_parsing() {
    let tests = vec![
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = assert_no_parse_errors(parser.parse_program());

        let expr = program.statements[0]
            .as_expression()
            .expect("program.statements[0] is not ExpressionStatement");

        match expr {
            Expression::FunctionLiteral {
                parameters,
                ..
            } => {
                assert_eq!(
                    parameters.len(),
                    expected.len(),
                    "length parameters wrong. want {}, got={}",
                    expected.len(),
                    parameters.len()
                );

                for (i, ident) in parameters.iter().enumerate() {
                    assert_identifier(ident, expected[i]);
                }
            }

            _ => panic!("stmt.Expression is not a FunctionLiteral. got={}", expr),
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());

    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    let expr = program.statements[0]
        .as_expression()
        .expect("program.statements[0] is not ExpressionStatement");

    match expr {
        Expression::Call {
            function,
            arguments,
            ..
        } => {
            assert_identifier_expression(function, "add");

            assert_eq!(
                arguments.len(),
                3,
                "wrong length of arguments. got={}",
                arguments.len()
            );

            assert_integer_literal(&arguments[0], 1);
            assert_infix_expression(
                &arguments[1], 
                &Expression::IntegerLiteral {
                    token: Token { 
                        token_type: TokenType::Int, 
                        literal: "2".to_string() 
                    }, 
                    value: 2,
                },
                "*".to_string(),
                &Expression::IntegerLiteral {
                    token: Token { 
                        token_type: TokenType::Int,
                        literal: "3".to_string() 
                    },
                    value: 3,
                }
            );
            assert_infix_expression(
                &arguments[2], 
                &Expression::IntegerLiteral {
                    token: Token { 
                        token_type: TokenType::Int,
                        literal: "4".to_string() 
                    },
                    value: 4,
                },
                "+".to_string(),
                &Expression::IntegerLiteral {
                    token: Token { 
                        token_type: TokenType::Int,
                        literal: "5".to_string() 
                    },
                    value: 5,
                }
            );
        }

        _ => panic!("stmt.Expression is not a CallExpression. got={}", expr),
    }
}