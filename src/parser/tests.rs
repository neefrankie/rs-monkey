use crate::token::{TokenType};
use crate::ast::{Expression, Node, Program, Statement};
use crate::lexer::{Lexer};

use super::precedence::{Precedence};
use super::Parser;
use super::errors::ParseError;

#[test]
fn test_precedence() {
    let tests = vec![
        (TokenType::Eq, Precedence::Equal),
        (TokenType::NotEq, Precedence::Equal),
        (TokenType::LessThan, Precedence::LessGreater),
        (TokenType::GreaterThan, Precedence::LessGreater),
        (TokenType::Plus, Precedence::Sum),
        (TokenType::Minus, Precedence::Sum),
        (TokenType::Slash, Precedence::Product),
        (TokenType::Asterisk, Precedence::Product),
    ];

    for (token_type, prec) in tests {
        let got = Precedence::from_token(token_type).expect("unknown precedence");
        assert_eq!(
            got,
            prec,
        )
    }

    let tests = vec![
        (TokenType::Eq, Precedence::LessGreater),
        (TokenType::LessThan, Precedence::Sum),
        (TokenType::Plus, Precedence::Product),
        (TokenType::Slash, Precedence::Call),
    ];

    for (token_type, precedence) in tests {
        let got = Precedence::from_token(token_type).expect("unknown precedence");
        assert!(
            got < precedence,
        )
    }
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

    let ident = expr.as_identifier()
        .expect("expression is not an Identifier");

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

        let (
            infix_left, 
            infix_operator, 
            infix_right
        ) = expr.as_infix().expect("expression is not an InfixExpression");

        test_integer_literal(
            infix_left, 
            left_value
        );
        
        assert_eq!(
            infix_operator,
            expected_operator,
            "exp.operator is not {}. got={}",
            expected_operator,
            infix_operator
        );

        test_integer_literal(
            infix_right,
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