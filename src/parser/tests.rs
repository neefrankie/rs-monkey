use std::vec;

use crate::token::{Token, TokenType, token_from_str, new_int_token};
use crate::ast::{
    Expression, Statement, new_block_stmt, new_boolean_expr, new_expr_stmt, new_identifier, new_identifier_expr, new_infix_expr, new_integer_expr, new_string_expr
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
fn test_epxect_token() {
    let input = "[]";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    parser.expect_peek(TokenType::RightBracket).unwrap();
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

    assert_identifier_expr(&expr, "foobar");
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
fn test_parse_grouped_expression() {
    let input = "(5 + 5) * 2";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let expr = parser.parse_grouped_expression().unwrap();

    assert_eq!(
        parser.current_token,
        token_from_str(")")
    );

    parser.next_token();

    assert_eq!(parser.current_token, token_from_str("*"));

    let expr = parser.parse_infix_expression(expr).unwrap();

    assert_eq!(parser.current_token, token_from_str("2"));

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
fn test_parse_if_expression() {
    let input = "if (x < y) { x } else { y }";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let expr = parser.parse_if_expression().unwrap();
    assert_eq!(parser.current_token, token_from_str("}"));

    assert_if_expression(
        &expr, 
        &new_infix_expr(
            new_identifier_expr("x"),
            "<",
            new_identifier_expr("y")
        ),
        &new_block_stmt(vec![
            new_expr_stmt("x", new_identifier_expr("x"))
        ]),
        Some(
            &new_block_stmt(vec![
                new_expr_stmt("y", new_identifier_expr("y"))
            ])
        ),
    );
}

#[test]
fn test_parse_function_literal() {
    let input = "fn(x, y) { x + y; }";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let function = parser.parse_function_literal().unwrap();

    assert_function_literal(
        &function,
        &vec![
            new_identifier("x"),
            new_identifier("y"),
        ], 
        &new_block_stmt(vec![
            new_expr_stmt(
                "x",
                new_infix_expr(
                    new_identifier_expr("x"), 
                    "+", 
                    new_identifier_expr("y")
                )
            ),
        ]),
    );
}

#[test]
fn test_parse_function_parameters() {
    let input = "(x, y)";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let identifiers = parser.parse_function_parameters().unwrap();

    assert_eq!(identifiers.len(), 2);

    assert_identifier_value(
        &identifiers[0],
        "x",
    );

    assert_identifier_value(
        &identifiers[1],
        "y",
    );
}

#[test]
fn test_parse_array_literal() {
    // parse_array_literal()
    // parse_expression_list()
    //   if peek_token_is("]") end
    //   move to 1
    //   parse_expression() -> parse_integer()
    //   move to comma, move to 2
    //   parse_expression()
    //     parse_prefix_expression() <- 2
    //       parse_infix_expression() <- * 2
    //    ...
    //   expect "]" and move to it.
    let input = "[1, 2 * 2, 3 + 3]";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let array = parser.parse_array_literal().unwrap();

    assert_array_literal(
        &array, 
        &vec![
            new_integer_expr(1),
            new_infix_expr(
                new_integer_expr(2),
                "*",
                new_integer_expr(2),
            ),
            new_infix_expr(
                new_integer_expr(3),
                "+",
                new_integer_expr(3),
            ),
        ],
    );

    assert_eq!(parser.current_token, token_from_str("]"));
}

#[test]
fn test_parse_hash_literal() {
    let input = r#"{"one": 1, "two": 2}"#;
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let expr = parser.parse_hash_literal().unwrap();

    assert_hash_literal(
        &expr,
        &vec![
            (
                new_string_expr("one"),
                new_integer_expr(1),
            ),
            (
                new_string_expr("two"),
                new_integer_expr(2),
            ),
        ],
    );
}

#[test]
fn test_parse_infix_expression() {
    let input = "5 + 5;";
    let mut parser = Parser::new(Lexer::new(input.to_string()));

    let expr = parser.parse_integer().unwrap();

    assert_eq!(parser.current_token, new_int_token(5));

    parser.next_token();

    assert_eq!(parser.current_token, token_from_str("+"));

    let expr = parser.parse_infix_expression(expr).unwrap();

    assert_eq!(parser.current_token, new_int_token(5));

    assert_infix_expression(
        &expr,
        &new_integer_expr(5),
        "+",
        &new_integer_expr(5)
    );
}

#[test]
fn test_parse_call_expression() {
    let input = "add(2, 3);";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let function = parser.parse_identifier();
    parser.next_token();
    assert_eq!(parser.current_token, token_from_str("("));

    let expr = parser.parse_call_expression(function).unwrap();

    assert_call_expression(
        &expr,
        &new_identifier_expr("add"),
        &vec![
            new_integer_expr(2),
            new_integer_expr(3),
        ]
    );
}

#[test]
fn test_parse_expression_list() {
    let input = "(2, 3)";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let exprs = parser.parse_expression_list(TokenType::RightParen).unwrap();

    assert_eq!(exprs.len(), 2);

    assert_expr(
        &exprs[0],
        &new_integer_expr(2),
    );

    assert_expr(
        &exprs[1],
        &new_integer_expr(3),
    );

    assert_eq!(parser.current_token, token_from_str(")"));
}

#[test]
fn test_parse_array_index_expression() {
    let input = "myArray[1 + 1]";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let left = parser.parse_identifier();
    // If you don't call next_token(), 
    // you will get array literal
    parser.next_token();

    assert_eq!(parser.current_token, token_from_str("["));

    let expr = parser.parse_index_expression(left).unwrap();

    assert_index_expression(
        &expr, 
        &new_identifier_expr("myArray"), 
        &new_infix_expr(
            new_integer_expr(1), 
            "+",
            new_integer_expr(1),
            ),
    );
}

#[test]
fn test_parse_hash_key() {
    let input = r#"hash["key"]"#;
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let left = parser.parse_identifier();
    parser.next_token();

    let expr = parser.parse_index_expression(left).unwrap();

    assert_index_expression(
        &expr,
        &new_identifier_expr("hash"),
        &new_string_expr("key"),
    );
}

// === Integrated test ===

#[test]
fn test_index_precedence() {
    let tests = vec![
        (
            "a * [1, 2, 3, 4][b * c] * d", 
            "((a * ([1, 2, 3, 4][(b * c)])) * d)"
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
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
fn test_parse_block_stmt() {
    let input = "{ 5; }";
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let stmt = parser.parse_block_statement().unwrap();

    assert_eq!(parser.current_token, token_from_str("}"));

    assert_stmt(
        &stmt.statements[0],
        &new_expr_stmt(
            "5",
            new_integer_expr(5),
        ),
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

    assert_identifier_expr(ident, "x");

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

            assert_identifier_value(&parameters[0], "x");
            assert_identifier_value(&parameters[1], "y");

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
                    assert_identifier_value(ident, expected[i]);
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
            assert_identifier_expr(function, "add");

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