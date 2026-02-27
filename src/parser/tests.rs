use crate::token::{Token, TokenType};
use crate::ast::{Expression, Node, Program, Statement};
use crate::lexer::{Lexer};

use super::precedence::{Precedence};
use super::Parser;
use super::errors::ParseError;

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

fn test_let_statement(stmt: &Statement, expected_name: &str) -> bool {
    if stmt.token_literal() != "let".to_string() {
        println!("Expected 'let', got '{}'", stmt.token_literal());
        return false;
    }

    let (let_name, _) = stmt.as_let()
        .expect("stmt is not a LetStatement");

    if let_name.value != expected_name {
        eprintln!(
            "Statement.Let.name.value not '{}'. got '{}'", 
            expected_name,
            let_name.value
        );
        return false;
    }

    if let_name.token_literal() != expected_name {
        eprintln!(
            "Statement.Let.name.token_literal() not '{}', got '{}'", 
            expected_name,
            let_name.token_literal()
        );
        return false;
    }

    true
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

    for (i, &expected_identifier) in tests.iter().enumerate() {
        // &* 先解引用得到 dyn Statement，再取引用得到 &dyn Statement
        let stmt = &programm.statements[i];

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
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = assert_no_parse_errors(parser.parse_program());
    
    assert_eq!(program.statements.len(), 3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len());

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

    test_identifier(expr, "foobar".to_string());
}

fn test_identifier(exp: &Expression, value: String) -> bool {
    let Some(ident) = exp.as_identifier() else {
        eprintln!("exp is not an Expression::Identifier. got {}", exp);
        return false;
    };
        
    assert_eq!(
        ident.value,
        "foobar",
        "Identifier.value not {}. got={}",
        value,
        ident.value
    );

    assert_eq!(
        ident.token_literal(),
        value,
        "Identifier.token_literal not {}. got={}",
        value,
        ident.token_literal()
    );

    return true;
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

    let literal_value = expr.as_integral()
        .expect("expr is not IntegerLiteral");

    assert_eq!(
        literal_value,
        5,
        "IntegralLiteral.value is not {}, got {}",
        5,
        literal_value
    );

    assert_eq!(
        expr.token_literal(),
        "5",
        "IntegralLiteral.token_literal not {}. got={}",
        "5",
        expr.token_literal()
    );
}

#[test]
fn test_parsing_prefix_expressions() {
    let tests = vec![
        ("!5;", "!", 5),
        ("-15;", "-", 15),
    ];

    for (input, expected_operator, expected_value) in tests {
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

        let (prefix_op, prefix_right) = expr.as_prefix()
            .expect("expr is not Expression::Prefix");
        
        assert_eq!(
            prefix_op,
            expected_operator,
            "exp.Operator is not {}. got={}",
            expected_operator,
            prefix_op
        );

        test_integer_literal(
            prefix_right, 
            expected_value
        );
    }
}

fn test_integer_literal(il: &Expression, expected_value: i64) -> bool {

    let Some(integ) = il.as_integral() else {
        eprintln!(
            "il not Expression::IntegerLiteral. got={}",
            il
        );
        return false;
    };

    assert_eq!(
        integ,
        expected_value,
        "IntegralLiteral.value is not {}. got={}",
        expected_value,
        integ,
    );
    assert_eq!(
        il.token_literal(),
        integ.to_string(),
        "iteg.token_literal is not {}. got={}",
        integ,
        il.token_literal()
    );

    return true;
}

fn test_boolean_literal(exp: &Expression, value: bool) -> bool {
    let Some(bo) = exp.as_boolean() else {
        eprintln!(
            "exp not Expression::Boolean. got={}",
            exp
        );
        return false;
    };

    assert_eq!(
        bo,
        value,
        "Boolean.value is not {}. got={}",
        value,
        bo,
    );
    assert_eq!(
        exp.token_literal(),
        bo.to_string(),
        "bo.token_literal not {}. got={}",
        value,
        exp.token_literal()
    );

    return true;
}

fn test_literal_expression(exp: &Expression, expected: &Expression) -> bool {
    match expected {
        Expression::IntegerLiteral { value, .. } => {
            test_integer_literal(exp, *value)
        }
        Expression::Ident(identifier) => {
            test_identifier(exp, identifier.value.clone())
        }
        Expression::Boolean { value , ..} => {
            test_boolean_literal(exp, *value)

        }
        _ => {
            eprintln!("type of exp not handled. got {}", exp);
            false
        }
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

        test_infix_expression(
            expr, 
            left_value, 
            expected_operator.to_string(), 
            right_value
        );
    }
}

fn test_infix_expression(
    exp: &Expression, 
    left: &Expression, 
    operator: String, 
    right: &Expression
) -> bool {
    let Some((
        infix_left, 
        infix_operator, 
        infix_right
    )) = exp.as_infix() else {
        eprintln!("expression is not an Expression::Infix. got {}", exp);
        return false;
    };

    if !test_literal_expression(
        infix_left, 
        left
    ) {
        return false;
    }
    
    assert_eq!(
        infix_operator,
        operator,
        "exp.operator is not {}. got={}",
        operator,
        infix_operator
    );

    if !test_literal_expression(
        infix_right,
        right
    ) {
        return false;
    };

    return true;
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