use super::*;

#[test]
fn test_read_char() {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.position, 0);
    assert_eq!(lexer.ch, b'=');
    assert_eq!(lexer.peek_char(), b'+');

    lexer.read_char();
    assert_eq!(lexer.position, 1);
    assert_eq!(lexer.ch, b'+');
    assert_eq!(lexer.peek_char(), b'(');
    

    lexer.read_char();
    assert_eq!(lexer.position, 2);
    assert_eq!(lexer.ch, b'(');
    assert_eq!(lexer.peek_char(), b')');

    lexer.read_char();
    assert_eq!(lexer.position, 3);
    assert_eq!(lexer.ch, b')');
    assert_eq!(lexer.peek_char(), b'{');

    lexer.read_char();
    assert_eq!(lexer.position, 4);
    assert_eq!(lexer.ch, b'{');
    assert_eq!(lexer.peek_char(), b'}');

    lexer.read_char();
    assert_eq!(lexer.position, 5);
    assert_eq!(lexer.ch, b'}');
    assert_eq!(lexer.peek_char(), b',');

    lexer.read_char();
    assert_eq!(lexer.position, 6);
    assert_eq!(lexer.ch, b',');
    assert_eq!(lexer.peek_char(), b';');

    lexer.read_char();
    assert_eq!(lexer.position, 7);
    assert_eq!(lexer.ch, b';');
    assert_eq!(lexer.peek_char(), b'\0');

    lexer.read_char();
    assert_eq!(lexer.position, 8);
    assert_eq!(lexer.ch, b'\0');
    assert_eq!(lexer.peek_char(), b'\0');
}

#[test]
fn test_read_identifier() {
    let input = "let fn five";
    let mut lexer = Lexer::new(input.to_string());

    let ident = lexer.read_identifier();
    assert_eq!(ident, "let");
    assert_eq!(lexer.position, 3);
    assert_eq!(lexer.ch, b' ');

    lexer.read_char();

    let ident = lexer.read_identifier();
    assert_eq!(ident, "fn");
    assert_eq!(lexer.position, 6);
    assert_eq!(lexer.ch, b' ');

    lexer.read_char();

    let ident = lexer.read_identifier();
    assert_eq!(ident, "five");
    assert_eq!(lexer.position, 11);
    assert_eq!(lexer.ch, b'\0');
}

#[test]
fn test_skip_whitespace() {
    let input = " \t\n";
    let mut lexer = Lexer::new(input.to_string());

    lexer.read_char();
    lexer.skip_whitespace();
    assert_eq!(lexer.position, 3);
    assert_eq!(lexer.ch, b'\0');
}

#[test]
fn test_read_number() {
    let input = "15;";
    let mut lexer = Lexer::new(input.to_string());

    let number = lexer.read_number();
    assert_eq!(number, "15");
    assert_eq!(lexer.position, 2);
    assert_eq!(lexer.ch, b';');
}

#[test]
fn test_read_string() {
    let input = "\"hello world\"";
    let mut lexer = Lexer::new(input.to_string());

    let string = lexer.read_string();
    assert_eq!(string, "hello world");
    assert_eq!(lexer.position, 12);
    assert_eq!(lexer.ch, b'"');
}

#[test]
fn test_next_token() {
    

    let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
return true;
} else {
return false;
}

10 == 10;
10 != 9;
"foobar"
"foo bar"
"#;

    let tests = vec![
        (TokenType::Let, "let"),
        (TokenType::Ident, "five"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "ten"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "add"),
        (TokenType::Assign, "="),
        (TokenType::Function, "fn"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, ";"),
        (TokenType::RBrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "result"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "add"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "five"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "ten"),
        (TokenType::RParen, ")"),
        (TokenType::Semicolon, ";"),
        (TokenType::Bang, "!"),
        (TokenType::Minus, "-"),
        (TokenType::Slash, "/"),
        (TokenType::Asterisk, "*"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "5"),
        (TokenType::LessThan, "<"),
        (TokenType::Int, "10"),
        (TokenType::GreaterThan, ">"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::If, "if"),
        (TokenType::LParen, "("),
        (TokenType::Int, "5"),
        (TokenType::LessThan, "<"),
        (TokenType::Int, "10"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Return, "return"),
        (TokenType::True, "true"),
        (TokenType::Semicolon, ";"),
        (TokenType::RBrace, "}"),
        (TokenType::Else, "else"),
        (TokenType::LBrace, "{"),
        (TokenType::Return, "return"),
        (TokenType::False, "false"),
        (TokenType::Semicolon, ";"),
        (TokenType::RBrace, "}"),
        (TokenType::Int, "10"),
        (TokenType::Eq, "=="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "10"),
        (TokenType::NotEq, "!="),
        (TokenType::Int, "9"),
        (TokenType::Semicolon, ";"),
        (TokenType::String, "foobar"),
        (TokenType::String, "foo bar"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input.to_string());

    for (expected_type, expected_literal) in tests {
        let tok = lexer.next_token();
        assert_eq!(tok.token_type, expected_type);
        assert_eq!(tok.literal, expected_literal.to_string());
    }
}