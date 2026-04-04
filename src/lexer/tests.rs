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
fn test_skip_whitespace() {
    let input = " \t\n";
    let mut lexer = Lexer::new(input.to_string());

    lexer.read_char();
    lexer.skip_whitespace();
    assert_eq!(lexer.position, 3);
    assert_eq!(lexer.ch, b'\0');
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
fn test_equal_operator() {
    let input = "a == b";
    let mut lexer = Lexer::new(input.to_string());
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Eq,
        literal: "==".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_plus_operator() { 
    let input = "a + b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Plus,
        literal: "+".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_minus_operator() {
    let input = "a - b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Minus,
        literal: "-".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_not_equal_operator() {
    let input = "a != b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::NotEq,
        literal: "!=".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_negate_operator() {
    let input = "!5";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Bang,
        literal: "!".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "5".to_string(),
    })
}

#[test]
fn test_division_operator() {
    let input = "a / b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Slash,
        literal: "/".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_multiply_operator() {
    let input = "a * b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Asterisk,
        literal: "*".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_less_than_operator() {
    let input = "a < b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LessThan,
        literal: "<".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_greater_than_operator() {
    let input = "a > b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::GreaterThan,
        literal: ">".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_comma_operator() {
    let input = "a, b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Comma,
        literal: ",".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_colon_operator() {
    let input = "a: b";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Colon,
        literal: ":".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });
}

#[test]
fn test_parentheses() {
    let input = "(b)";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftParen,
        literal: "(".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "b".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightParen,
        literal: ")".to_string(),
    });
}

#[test]
fn test_braces() {
    let input = "{ a; }";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftBrace,
        literal: "{".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "a".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Semicolon,
        literal: ";".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightBrace,
        literal: "}".to_string(),
    });
}

#[test]
fn test_brackets() { 
    let input = "[1]";

    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftBracket,
        literal: "[".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "1".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightBracket,
        literal: "]".to_string(),
    });
}

#[test]
fn test_string_literal() {
    let input = r#"let foo = "foobar";"#;

    let mut lexer = Lexer::new(input.to_string());
    
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Let,
        literal: "let".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "foo".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Assign,
        literal: "=".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::String,
        literal: "foobar".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Semicolon,
        literal: ";".to_string(),
    });
}

#[test]
fn test_identifier() {
    let input = "fn let true false if else return x";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Function,
        literal: "fn".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Let,
        literal: "let".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::True,
        literal: "true".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::False,
        literal: "false".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::If,
        literal: "if".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Else,
        literal: "else".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Return,
        literal: "return".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "x".to_string(),
    });

}

#[test]
fn test_let_statement() {
    let input = "let five = 5;";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token{
        token_type: TokenType::Let,
        literal: "let".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "five".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Assign,
        literal: "=".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "5".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Semicolon,
        literal: ";".to_string(),
    });

    assert_eq!(lexer.position, 13);
    assert_eq!(lexer.ch, b'\0');
    assert_eq!(lexer.read_position, 14);
}

#[test]
fn test_function_literal() {
    let input = "fn(x, y) { x + y; }";

    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Function,
        literal: "fn".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftParen,
        literal: "(".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "x".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Comma,
        literal: ",".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "y".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightParen,
        literal: ")".to_string(),
    });
}

#[test]
fn test_function_call() {
    let input = "add(5, 10)";

    let mut lexer = Lexer::new(input.to_string());
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "add".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftParen,
        literal: "(".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "5".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Comma,
        literal: ",".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "10".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightParen,
        literal: ")".to_string(),
    });
}

#[test]
fn test_array_literal() {
    let input = "[1, 2]";

    let mut lexer = Lexer::new(input.to_string());
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftBracket,
        literal: "[".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "1".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Comma,
        literal: ",".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "2".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightBracket,
        literal: "]".to_string(),
    });
}
#[test]
fn test_array_index() {
    let input = "my_array[1]";
    let mut lexer = Lexer::new(input.to_string());
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "my_array".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftBracket,
        literal: "[".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "1".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightBracket,
        literal: "]".to_string(),
    });
}

#[test]
fn test_hash_literal() {
    let input = r#"{"one": 1}"#;

    let mut lexer = Lexer::new(input.to_string());
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftBrace,
        literal: "{".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::String,
        literal: "one".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Colon,
        literal: ":".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Int,
        literal: "1".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightBrace,
        literal: "}".to_string(),
    });
}

#[test]
fn test_hash_index() {
    let input = r#"hash_map["one"]"#;
    let mut lexer = Lexer::new(input.to_string());
    
    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::Ident,
        literal: "hash_map".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::LeftBracket,
        literal: "[".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::String,
        literal: "one".to_string(),
    });

    assert_eq!(lexer.next_token(), token::Token {
        token_type: TokenType::RightBracket,
        literal: "]".to_string(),
    });
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
[1, 2];
{"foo": "bar"}
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
        (TokenType::LeftParen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::RightParen, ")"),
        (TokenType::LeftBrace, "{"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, ";"),
        (TokenType::RightBrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "result"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "add"),
        (TokenType::LeftParen, "("),
        (TokenType::Ident, "five"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "ten"),
        (TokenType::RightParen, ")"),
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
        (TokenType::LeftParen, "("),
        (TokenType::Int, "5"),
        (TokenType::LessThan, "<"),
        (TokenType::Int, "10"),
        (TokenType::RightParen, ")"),
        (TokenType::LeftBrace, "{"),
        (TokenType::Return, "return"),
        (TokenType::True, "true"),
        (TokenType::Semicolon, ";"),
        (TokenType::RightBrace, "}"),
        (TokenType::Else, "else"),
        (TokenType::LeftBrace, "{"),
        (TokenType::Return, "return"),
        (TokenType::False, "false"),
        (TokenType::Semicolon, ";"),
        (TokenType::RightBrace, "}"),
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
        (TokenType::LeftBracket, "["),
        (TokenType::Int, "1"),
        (TokenType::Comma, ","),
        (TokenType::Int, "2"),
        (TokenType::RightBracket, "]"),
        (TokenType::Semicolon, ";"),
        (TokenType::LeftBrace, "{"),
        (TokenType::String, "foo"),
        (TokenType::Colon, ":"),
        (TokenType::String, "bar"),
        (TokenType::RightBrace, "}"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input.to_string());

    for (expected_type, expected_literal) in tests {
        let tok = lexer.next_token();
        assert_eq!(tok.token_type, expected_type);
        assert_eq!(tok.literal, expected_literal.to_string());
    }
}