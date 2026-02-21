use crate::token::{self, TokenType};

pub struct Lexer {
    input: Vec<u8>,
    position: usize, // 指向所输入字符串中与ch字节对应的字符
    read_position: usize, // 始终指向所输入字符串中的下一个字符
    ch: u8,
}

impl Lexer { 
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char(); // Move to first character
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // 或者用 b'\0'
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok = match self.ch {
            b'=' => new_token(TokenType::Assign, self.ch),
            b';' => new_token(TokenType::Semicolon, self.ch),
            b'(' => new_token(TokenType::LParen, self.ch),
            b')' => new_token(TokenType::RParen, self.ch),
            b',' => new_token(TokenType::Comma, self.ch),
            b'+' => new_token(TokenType::Plus, self.ch),
            b'{' => new_token(TokenType::LBrace, self.ch),
            b'}' => new_token(TokenType::RBrace, self.ch),
            0 => token::Token { token_type: TokenType::Eof, literal: String::new() },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                }
                new_token(TokenType::Illegal, self.ch)
            },
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }
}

fn new_token(kind: token::TokenType, ch: u8) -> token::Token {
    token::Token {
        token_type: kind,
        literal: char::from(ch).to_string(),
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}


mod tests {
    
    #[test]
    fn test_next_token() {
        use super::*;

        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
	x + y;
};

let result = add(five, ten);
!-/*5
5 < 10 > 5

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
{"foo": "bar"}"#;

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
            (TokenType::Ident, "="),
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
            (TokenType::Ident, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (expected_type, expected_literal) in tests {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal.to_string());
        }
    }
}
