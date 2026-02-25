use crate::token::{TokenType};
use crate::ast::{self, Node};
use crate::lexer;

use super::precedence::*;
use super::Parser;
use super::errors::*;

#[test]
fn test_precedence() {
    let tests = vec![
        (TokenType::Eq, EQUAL),
        (TokenType::NotEq, EQUAL),
        (TokenType::LessThan, LESSGRATER),
        (TokenType::GreaterThan, LESSGRATER),
        (TokenType::Plus, SUM),
        (TokenType::Minus, SUM),
        (TokenType::Slash, PRODUCT),
        (TokenType::Asterisk, PRODUCT),
    ];

    for (token_type, precedence) in tests {
        assert_eq!(
            token_precedence(token_type),
            precedence,
        )
    }

    let tests = vec![
        (TokenType::Eq, LESSGRATER),
        (TokenType::LessThan, SUM),
        (TokenType::Plus, PRODUCT),
        (TokenType::Slash, CALL),
    ];

    for (token_type, precedence) in tests {
        assert!(
            token_precedence(token_type) < precedence,
        )
    }
}

fn as_let_statement(stmt: &dyn ast::Statement) -> Option<&ast::LetStatement> {
    stmt.as_any().downcast_ref::<ast::LetStatement>()
}

// fn assert_let_statement(stmt: &dyn ast::Statement) -> &ast::LetStatement {
//     as_let_statement(stmt).expect("expected LetStatement")
// }

fn as_return_statement(stmt: &dyn ast::Statement) -> Option<&ast::ReturnStatement> {
    stmt.as_any().downcast_ref::<ast::ReturnStatement>()
}

fn as_identifier(expr: &dyn ast::Expression) -> Option<&ast::Identifier> {
    expr.as_any().downcast_ref::<ast::Identifier>()
}

fn assert_identifier(expr: &dyn ast::Expression) -> &ast::Identifier {
    as_identifier(expr).expect("expected Identifier")
}

fn as_expression_statement(stmt: &dyn ast::Statement) -> Option<&ast::ExpressionStatement> {
    stmt.as_any().downcast_ref::<ast::ExpressionStatement>()
}

fn assert_expression_statement(stmt: &dyn ast::Statement) -> &ast::ExpressionStatement {
    as_expression_statement(stmt).expect("expected ExpressionStatement")
}

fn assert_integral_literal(expr: &dyn ast::Expression) -> &ast::IntegerLiteral {
    expr.as_any()
        .downcast_ref::<ast::IntegerLiteral>()
        .expect("Expected IntergralLiteral")
}

fn assert_prefix_expression(expr: &dyn ast::Expression) -> &ast::PrefixExpression {
    expr.as_any()
        .downcast_ref::<ast::PrefixExpression>()
        .expect("Expected PrefixExpression")
}

fn assert_infix_expression(expr: &dyn ast::Expression) -> &ast::InfixExpression {
    expr.as_any()
        .downcast_ref::<ast::InfixExpression>()
        .expect("Expected InfixExpression")
}

fn test_let_statement(stmt: &dyn ast::Statement, name: &str) -> bool {
    if stmt.token_literal() != "let".to_string() {
        println!("Expected 'let', got '{}'", stmt.token_literal());
        return false;
    }

    let let_stmt = match as_let_statement(stmt) {
        Some(let_stmt) => let_stmt,
        None => {
            println!("Statement is not a LetStatement");
            return false;
        }
    };

    if let_stmt.name.value != name {
        println!("Expected '{}', got '{}'", name, let_stmt.name.value);
        return false;
    }

    if let_stmt.name.token.literal != name {
        println!("Expected '{}', got '{}'", name, let_stmt.name.token.literal);
        return false;
    }

    true
}

fn assert_no_parse_errors(result: Result<ast::Program, Vec<ParseError>>) -> ast::Program {
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

// fn parse_without_errors(input: &str) -> Program {
//     let lex = lexer::Lexer::new(input.to_string());
//     let mut parser = Parser::new(lex);
//     assert_no_parse_errors(parser.parse_program())
// }

// fn assert_parse_error(input: &str, expected_error_count: usize) {
//     let lex = lexer::Lexer::new(input.to_string());
//     let mut parser = Parser::new(lex);
//     let result = parser.parse_program();
    
//     match result {
//         Ok(_) => panic!("Expected parse errors but got success"),
//         Err(errors) => {
//             assert_eq!(errors.len(), expected_error_count, 
//                     "Expected {} errors, got {}", expected_error_count, errors.len());
//             // 可选：打印错误用于调试
//             for error in &errors {
//                 eprintln!("Error: {:?}", error);
//             }
//         }
//     }
// }

#[test]
fn test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;
";
    let lex = lexer::Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let programm = assert_no_parse_errors(parser.parse_program());

    assert_eq!(programm.statements.len(), 3,
        "program.statements does not contain 3 statements. got={}",
        programm.statements.len());

    let tests = vec![
        "x",
        "y",
        "foobar",
    ];

    for (i, &expected_identifier) in tests.iter().enumerate() {
        // &* 先解引用得到 dyn Statement，再取引用得到 &dyn Statement
        let stmt = &*programm.statements[i];

        assert!(
            test_let_statement(stmt, expected_identifier),
            "Test failed for statement {} with identifier {}",
            i,
            expected_identifier
        )
    }
}

#[test]
fn test_return_statements() {
    let input = "return 5;
return 10;
return 993 322;
";
    let lex = lexer::Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());
    
    assert_eq!(program.statements.len(), 3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len());

    for box_stmt in program.statements {
        let stmt = &*box_stmt;

        let ret_stmt = match as_return_statement(stmt) {
            Some(ret_stmt) => ret_stmt,
            None => {
                println!("Statement is not a ReturnStatement");
                continue;
            }
        };

        assert_eq!(
            ret_stmt.token_literal(),
            "return",
            "return statement does not 'return', got {}",
            ret_stmt.token_literal()
        )
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lex = lexer::Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());

    assert_eq!(program.statements.len(), 1,
        "program has not enough statements. got={}",
        program.statements.len());

    let stmt = &*(program.statements[0]);
    let expr_stmt = assert_expression_statement(stmt);
    let ident = assert_identifier(&*expr_stmt.expression);

    assert_eq!(
        ident.value,
        "foobar",
        "Identifier has wrong value. got={}",
        ident.value
    );

    assert_eq!(
        ident.token_literal(),
        "foobar",
        "Identifier has wrong token_literal. got={}",
        ident.token_literal()

    );
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let lex = lexer::Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());

    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got={}",
        program.statements.len()
    );

    let stmt = assert_expression_statement(&*(program.statements[0]));
    let literal = assert_integral_literal(&*stmt.expression);
    assert_eq!(
        literal.value,
        5,
        "Literal has wrong value. got={}",
        literal.value
    );

    assert_eq!(
        literal.token_literal(),
        "5",
        "Literal has wrong token_literal. got={}",
        literal.token_literal()
    );
}

#[test]
fn test_parsing_prefix_expressions() {
    let tests = vec![
        ("!5;", "!", 5),
        ("-15;", "-", 15),
    ];

    for (input, operator, value) in tests {
        let lex = lexer::Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = assert_no_parse_errors(parser.parse_program());

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        let stmt = assert_expression_statement(&*(program.statements[0]));
        let exp = assert_prefix_expression(&*stmt.expression);
        assert_eq!(
            exp.operator,
            operator,
            "exp.Operator is not {}. got={}",
            operator,
            exp.operator
        );

        let iteg = assert_integral_literal(&*exp.right);
        test_integer_literal(iteg, value);
    }
}

fn test_integer_literal(iteg: &ast::IntegerLiteral, value: i64) { 
    assert_eq!(
        iteg.value,
        value,
        "iteg.value is not {}. got={}",
        value,
        iteg.value
    );
    assert_eq!(
        iteg.token_literal(),
        iteg.value.to_string(),
        "iteg.token_literal is not {}. got={}",
        iteg.value,
        iteg.token_literal()
    );
}

#[test]
fn test_parsing_infix_expressions() {
    let tests = vec![
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
    ];

    for (input, left_value, operator, right_value) in tests {
        let lex = lexer::Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = assert_no_parse_errors(parser.parse_program());
        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );
        let stmt = assert_expression_statement(&*(program.statements[0]));
        let exp = assert_infix_expression(&*stmt.expression);
        test_integer_literal(assert_integral_literal(&*exp.left), left_value);
        
        assert_eq!(
            exp.operator,
            operator,
            "exp.operator is not {}. got={}",
            operator,
            exp.operator
        );

        test_integer_literal(assert_integral_literal(&*exp.right), right_value);
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
    ];

    for (input, expected) in tests {
        let l = lexer::Lexer::new(input.to_string());
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