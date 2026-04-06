use crate::token::{Token, TokenType};
use crate::ast::{
    Expression,
    Node,
    Statement,
    new_boolean_expr,
    new_identifier_expr,
    new_integer_expr,
    new_infix_expr,
};
use crate::lexer::{Lexer};

use super::precedence::{Precedence};
use super::*;


#[test]
fn test_precedence() {
    assert!(Precedence::Lowest < Precedence::Equal);
    assert!(Precedence::Equal < Precedence::LessGreater);
    assert!(Precedence::LessGreater < Precedence::Sum);
    assert!(Precedence::Sum < Precedence::Product);
    assert!(Precedence::Product < Precedence::Prefix);
    assert!(Precedence::Prefix < Precedence::Call);
    assert!(Precedence::Call < Precedence::Index)
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
        Precedence::from_token(TokenType::LeftBrace),
        None
    );
}

#[test]
fn test_new_parser() {
    let input = "let x = 5;";
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    
    assert_eq!(parser.current_token, Token {
        token_type: TokenType::Let,
        literal: "let".to_string(),
    });

    assert_eq!(parser.peek_token, Token {
        token_type: TokenType::Ident,
        literal: "x".to_string(),
    });

    assert_eq!(parser.lexer.current_position(), 5);
}

#[test]
fn test_next_token() {
    let input = "let x = 5;";

    let tests = vec![
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Eof,
            literal: "".to_string(),
        },
    ];

    let mut parser = Parser::new(Lexer::new(input.to_string()));

    for expected in tests {
        assert_eq!(parser.current_token, expected);
        if expected.token_type == TokenType::Eof {
            break;
        }
        parser.next_token();
    }

    assert_eq!(parser.lexer.current_position(), 12);
}

#[test]
fn test_parse_identifier() {
    let input = "foobar;";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    // current token is foobar
    // peek token is ;
    // lexer.position == 7
    assert_eq!(parser.current_token, Token {
        token_type: TokenType::Ident,
        literal: "foobar".to_string(),
    });

    let expr = parser.parse_identifier();

    assert_identifier_expression(&expr, "foobar");
}

#[test]
fn test_parse_integer() {
    let input = "5;";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    assert_eq!(parser.current_token, Token {
        token_type: TokenType::Int,
        literal: "5".to_string(),
    });

    let expr = parser.parse_integer().unwrap();

    assert_integer_literal(&expr, 5);
}

#[test]
fn test_parse_boolean() {
    let input = "true;";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    assert_eq!(parser.current_token, Token {
        token_type: TokenType::True,
        literal: "true".to_string(),
    });

    let expr = parser.parse_boolean();

    assert_boolean(&expr, true);
}

#[test]
fn test_parse_string_literal() {
    let input = "\"hello world\";";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    assert_eq!(parser.current_token, Token {
        token_type: TokenType::String,
        literal: "hello world".to_string(),
    });

    let expr = parser.parse_string_literal();

    assert_string(&expr, "hello word");
}

#[test]
fn test_parse_prefix_expression() {
    let input = "!5;
-15;
!foobar;
-foobar;
";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let expr = parser.parse_prefix_expression().unwrap();
    assert_prefix_expression(
        &expr, 
        "!", 
        &new_integer_expr(5)
    );

    parser.next_token(); // skip ;
    parser.next_token();

    let expr = parser.parse_prefix_expression().unwrap();
    assert_prefix_expression(
        &expr,
        "-",
        &new_integer_expr(15)
    );

    parser.next_token();
    parser.next_token();

    let expr = parser.parse_prefix_expression().unwrap();
    assert_prefix_expression(
        &expr,
        "!",
        &new_identifier_expr("foobar")
    );

    parser.next_token();
    parser.next_token();

    let expr = parser.parse_prefix_expression().unwrap();
    assert_prefix_expression(
        &expr,
        "-",
        &new_identifier_expr("foobar")
    );
}

#[test]
fn test_parse_infix_expression() {
    let input = "5 + 5;";
    let mut parser = Parser::new(Lexer::new(input.to_string()));

    let expr = parser.parse_integer().unwrap();

    assert_integer_literal(&expr, 5);

    parser.next_token();

    let expr = parser.parse_infix_expression(expr).unwrap();

    assert_infix_expression(
        &expr,
        &new_integer_expr(5),
        "+",
        &new_integer_expr(5)
    );
}

#[test]
fn test_parse_grouped_expression() {
    let input = "(5 + 5) * 2";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let expr = parser.parse_grouped_expression().unwrap();

    parser.next_token();

    let expr = parser.parse_infix_expression(expr).unwrap();

    assert_infix_expression(
        &expr, 
        &new_infix_expr(
            new_integer_expr(5), 
            "+", 
            new_integer_expr(5)
        ), 
        "*", 
        &new_integer_expr(2)
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
    let programm = unwrap_program(parser.parse_program());

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
    let program = unwrap_program(parser.parse_program());
    
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
fn test_parsing_prefix_expressions() {
    let five = Expression::IntegerLiteral {
         token: Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
         }, 
         value: 5,
    };

    let fifteen = new_integer_expr(15);

    let bool_true = new_boolean_expr(true);

    let bool_false = new_boolean_expr(false);

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
        let program = unwrap_program(parser.parse_program());

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
    let five = new_integer_expr(5);

    let bool_true = new_boolean_expr(true);

    let bool_false = new_boolean_expr(false);

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
        let program = unwrap_program(parser.parse_program());
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
            expected_operator, 
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
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        ("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        ("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))")

    ];

    for (input, expected) in tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = unwrap_program(p.parse_program());

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
    let program = unwrap_program(parser.parse_program());

    assert_statements_len(&program, 1);

    // program.statements[0].expression
    let expr = unwrap_expression_statement(&program.statements[0]);

    let Expression::If {
        condition, 
        consequence, 
        alternative ,
        ..
    } = expr else {
        panic!("expr is not an IfExpression")
    };

    assert_infix_expression(
        condition,
        &new_identifier_expr("x"), 
        "<", 
        &new_identifier_expr("y"),
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
    let program = unwrap_program(parser.parse_program());

    assert_statements_len(&program, 1);

    let expr = unwrap_expression_statement(&program.statements[0]);

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
                        &new_identifier_expr("x"), 
                        "+", 
                        &new_identifier_expr("y"),
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
        let program = unwrap_program(parser.parse_program());

        let expr = unwrap_expression_statement(&program.statements[0]);

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
    let program = unwrap_program(parser.parse_program());

    assert_statements_len(&program, 1);

    let expr = unwrap_expression_statement(&program.statements[0]);

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
                &new_integer_expr(2),
                "*",
                &new_integer_expr(3)
            );
            assert_infix_expression(
                &arguments[2], 
                &new_integer_expr(4),
                "+",
                &new_integer_expr(5)
            );
        }

        _ => panic!("stmt.Expression is not a CallExpression. got={}", expr),
    }
}


#[test]
fn test_parsing_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = unwrap_program(parser.parse_program());
    let expr = unwrap_expression_statement(&program.statements[0]);
    
    match expr {
        Expression::ArrayLiteral {
            elements,
            ..
        } => {
            assert_eq!(elements.len(), 3);
            assert_integer_literal(&elements[0], 1);
            assert_infix_expression(
                &elements[1],
                &new_integer_expr(2),
                "*",
                &new_integer_expr(2)
            );
            assert_infix_expression(
                &elements[2],
                &new_integer_expr(3),
                "+",
                &new_integer_expr(3)
            );
        }

        _ => panic!("expression is not ArrayLiteral. got={}", expr),
    }
}