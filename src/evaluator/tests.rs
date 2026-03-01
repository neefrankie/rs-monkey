use crate::lexer::*;
use crate::parser::*;
use crate::object::*;
use super::*;

fn assert_eval(input: &str) -> Object {
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = parser.parse_program().expect("parse program failed");
    let mut env = Environment::new();

    return eval_program(program, &mut env);
}

fn assert_integer_object(obj: &Object, expected: i64) {
    match obj {
        Object::Integer(value) => {
            assert_eq!(
                *value,
                expected,
                "object has wrong value. got {}, want {}",
                *value,
                expected
            );
        },
        _ => panic!("object is not Integer. got {}", obj),
    }
}

fn assert_boolean_object(obj: &Object, expected: bool) {
    match obj {
        Object::Boolean(value) => {
            assert_eq!(
                *value,
                expected,
                "object has wrong value. got {}, want {}",
                *value,
                expected
            );
        },
        _ => panic!("object is not Boolean. got {}", obj),
    }
}

fn assert_null_object(obj: &Object) {
    match obj {
        Object::Null => {},
        _ => panic!("object is not Null. got {}", obj),
    }
}

#[test]
fn test_eval_integer_expression() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        assert_integer_object(&evaluated, expected);
    }
}

#[test]
fn test_evaluate_boolean_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("true == false", false),
        ("true != true", false),
        ("false == false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        assert_boolean_object(&evaluated, expected);
    }
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        assert_boolean_object(&evaluated, expected);
    }
}

#[test]
fn test_if_else_expression() {
    let tests = vec![
        ("if (true) { 10 }", Object::Integer(10)),
        ("if (false) { 10 }", Object::Null),
        ("if (1) { 10 }", Object::Integer(10)),
        ("if (1 < 2) { 10 }", Object::Integer(10)),
        ("if (1 > 2) { 10 }", Object::Null),
        ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
        ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        match expected {
            Object::Integer(expected) => assert_integer_object(&evaluated, expected),
            _ => assert_null_object(&evaluated),
        }
    }
}

#[test]
fn test_return_statement() {
    let tests = vec![
        ("return 10;",10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            "if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }",
            10
        )
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        assert_integer_object(&evaluated, expected);
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN"),
        ("if (10 > 1) {
            if (10 > 1) {
                return true + false;
            }
            return 1;
        }", "unknown operator: BOOLEAN + BOOLEAN"),
        ("foobar", "identifier not found: foobar"),
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        match evaluated {
            Object::Error(message) => assert_eq!(
                message,
                expected,
                "wrong error message. expected {}, got {}",
                expected,
                message
            ),
            _ => panic!("no error object returned. got{}", evaluated),
        }
    }
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        let evaluated = assert_eval(input);
        assert_integer_object(&evaluated, expected);
    }
}